use crate::models::{FolderInfo, Resource};
use crate::settings::Settings;
use anyhow::{anyhow, Result};
use chrono::{DateTime, SecondsFormat, Utc};
use rusqlite::{params, Connection, Row};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Database {
    connection: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(settings: &Settings) -> Result<Self> {
        // Create the db directory if it doesn't exist
        if let Some(parent) = Path::new(&settings.db_file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&settings.db_file_path)?;

        // Initialize the database schema only if needed
        Self::initialize_schema(&conn, &settings.db_schema_path)?;

        Ok(Self {
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    fn initialize_schema(conn: &Connection, schema_path: &str) -> Result<()> {
        // Check if the resources table exists
        let table_exists: bool = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='resources'",
            [],
            |row| {
                let count: i64 = row.get(0)?;
                Ok(count > 0)
            },
        )?;

        if !table_exists {
            println!("Initializing database schema...");
            let schema = fs::read_to_string(schema_path)?;
            conn.execute_batch(&schema)?;
            println!("Database schema initialized successfully");
        } else {
            println!("Database schema already exists, skipping initialization");
        }

        Ok(())
    }

    pub fn create_resource(&self, resource: &Resource) -> Result<i64> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Failed to acquire database lock"))?;

        let mut stmt = conn.prepare(
            "INSERT INTO resources (user_id, path, content, size, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;

        let id = stmt.insert(params![
            resource.user_id,
            resource.path,
            resource.content,
            resource.size,
            resource
                .created_at
                .to_rfc3339_opts(SecondsFormat::Millis, true)
                .to_string(),
            resource
                .updated_at
                .to_rfc3339_opts(SecondsFormat::Millis, true)
                .to_string(),
        ])?;

        Ok(id)
    }

    pub fn get_resource(&self, path: &str) -> Result<Option<Resource>> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Failed to acquire database lock"))?;

        let mut stmt = conn.prepare(
            "SELECT id, user_id, path, content, size, created_at, updated_at 
             FROM resources WHERE path = ?1",
        )?;

        let result = stmt.query_row(params![path], |row| {
            // Better error handling - let's see exactly what's causing the issue
            match self.row_to_resource(row) {
                Ok(resource) => Ok(resource),
                Err(e) => {
                    // Log the actual error to help debug
                    eprintln!("Error converting row to resource: {}", e);
                    Err(rusqlite::Error::InvalidColumnType(
                        0,
                        format!("conversion error: {}", e),
                        rusqlite::types::Type::Null,
                    ))
                }
            }
        });

        match result {
            Ok(resource) => Ok(Some(resource)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow!("Database query error: {}", e)),
        }
    }

    pub fn update_resource(&self, path: &str, content: &str) -> Result<()> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Failed to acquire database lock"))?;

        let size = content.len() as i64;
        let updated_at = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true).to_string();

        let mut stmt = conn.prepare(
            "UPDATE resources SET content = ?1, size = ?2, updated_at = ?3 WHERE path = ?4",
        )?;

        let rows_affected = stmt.execute(params![content, size, updated_at, path])?;

        if rows_affected == 0 {
            return Err(anyhow!("Resource not found"));
        }

        Ok(())
    }

    pub fn delete_resource(&self, path: &str) -> Result<()> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Failed to acquire database lock"))?;

        let mut stmt = conn.prepare("DELETE FROM resources WHERE path = ?1")?;
        let rows_affected = stmt.execute(params![path])?;

        if rows_affected == 0 {
            return Err(anyhow!("Resource not found"));
        }

        Ok(())
    }

    pub fn list_folder_resources(&self, folder_path: &str) -> Result<FolderInfo> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Failed to acquire database lock"))?;

        // Normalize folder path
        let normalized_folder = if folder_path == "/" {
            "".to_string()
        } else {
            folder_path.trim_end_matches('/').to_string()
        };

        // Get all resources that start with the folder path
        let mut stmt = conn.prepare(
            "SELECT path, created_at FROM resources 
             WHERE (path LIKE ?1 OR path = ?2)
             ORDER BY path",
        )?;

        let pattern = format!("{}/%", normalized_folder);
        let rows = stmt.query_map(params![pattern, normalized_folder], |row| {
            let path: String = row.get(0)?;
            let created_at_str: String = row.get(1)?;
            Ok((path, created_at_str))
        })?;

        let mut resources = Vec::new();
        let mut folder_created_at = Utc::now();
        let mut found_folder = false;

        for row in rows {
            let (path, created_at_str) = row?;

            // If this is exactly the folder we're looking for
            if path == normalized_folder {
                found_folder = true;
                folder_created_at = self.parse_datetime(&created_at_str)?;
                continue;
            }

            // Check if this resource is directly under the requested folder
            let remaining_path = if normalized_folder.is_empty() {
                &path[1..] // Remove leading slash for root folder
            } else {
                &path[normalized_folder.len() + 1..] // Remove folder path and slash
            };

            if !remaining_path.is_empty() {
                resources.push(path);
            }
        }

        // If we're looking for root folder and haven't found it explicitly, create it
        if !found_folder && normalized_folder.is_empty() {
            found_folder = true;
        }

        if !found_folder && resources.is_empty() {
            return Err(anyhow!("Folder not found"));
        }

        Ok(FolderInfo {
            path: folder_path.to_string(),
            created_at: folder_created_at,
            resources,
        })
    }

    pub fn resource_exists(&self, path: &str) -> Result<bool> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Failed to acquire database lock"))?;

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM resources WHERE path = ?1")?;
        let count: i64 = stmt.query_row(params![path], |row| row.get(0))?;

        Ok(count > 0)
    }

    pub fn folder_is_empty(&self, folder_path: &str) -> Result<bool> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Failed to acquire database lock"))?;

        let normalized_folder = folder_path.trim_end_matches('/');
        let pattern = format!("{}/%", normalized_folder);

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM resources WHERE path LIKE ?1")?;
        let count: i64 = stmt.query_row(params![pattern], |row| row.get(0))?;

        Ok(count == 0)
    }

    fn row_to_resource(&self, row: &Row) -> Result<Resource> {
        // Get values with explicit error handling
        let id: i64 = row.get(0).map_err(|e| anyhow!("Failed to get id: {}", e))?;
        let user_id: i64 = row.get(1).map_err(|e| anyhow!("Failed to get user_id: {}", e))?;
        let path: String = row.get(2).map_err(|e| anyhow!("Failed to get path: {}", e))?;
        let content: Option<String> = row.get(3).map_err(|e| anyhow!("Failed to get content: {}", e))?;
        let size: i64 = row.get(4).map_err(|e| anyhow!("Failed to get size: {}", e))?;
        let created_at_str: String = row.get(5).map_err(|e| anyhow!("Failed to get created_at: {}", e))?;
        let updated_at_str: String = row.get(6).map_err(|e| anyhow!("Failed to get updated_at: {}", e))?;

        Ok(Resource {
            id: Some(id),
            user_id,
            path,
            content,
            size,
            created_at: self.parse_datetime(&created_at_str)?,
            updated_at: self.parse_datetime(&updated_at_str)?,
        })
    }

    fn parse_datetime(&self, datetime_str: &str) -> Result<DateTime<Utc>> {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => Ok(dt.with_timezone(&Utc)),
            Err(e) => Err(anyhow!("Failed to parse RFC3339 datetime: {}", e)),
        }
    }
}
