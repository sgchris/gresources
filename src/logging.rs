use anyhow::Result;
use log::error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Logger {
    log_file: Mutex<std::fs::File>,
}

impl Logger {
    pub fn new() -> Result<Self> {
        let log_dir = Self::get_log_directory()?;
        std::fs::create_dir_all(&log_dir)?;

        let log_file_path = log_dir.join("gresources.log");
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)?;

        Ok(Self {
            log_file: Mutex::new(file),
        })
    }

    pub fn log_write_operation(&self, operation: &str, path: &str, success: bool) {
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S%.3f")
            .to_string();
        let status = if success { "SUCCESS" } else { "FAILED" };
        let log_entry = format!("[{}] {} {} - {}\n", timestamp, operation, path, status);

        if let Ok(mut file) = self.log_file.lock() {
            if let Err(e) = file.write_all(log_entry.as_bytes()) {
                error!("Failed to write to log file: {}", e);
            } else {
                let _ = file.flush();
            }
        }

        // Also log to console (simple version)
        println!("{} {} - {}", operation, path, status);
    }

    // Enhanced method for detailed logging to file
    pub fn log_detailed(&self, level: &str, message: &str) {
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S%.3f")
            .to_string();
        let log_entry = format!("[{}] {} {}\n", timestamp, level, message);

        if let Ok(mut file) = self.log_file.lock() {
            if let Err(e) = file.write_all(log_entry.as_bytes()) {
                error!("Failed to write to log file: {}", e);
            } else {
                let _ = file.flush();
            }
        }
    }

    pub fn log_info(&self, message: &str) {
        self.log_detailed("INFO", message);
    }

    pub fn log_debug(&self, message: &str) {
        self.log_detailed("DEBUG", message);
    }

    pub fn log_warn(&self, message: &str) {
        self.log_detailed("WARN", message);
    }

    pub fn log_error(&self, message: &str) {
        self.log_detailed("ERROR", message);
    }

    #[cfg(target_os = "windows")]
    fn get_log_directory() -> Result<PathBuf> {
        let local_app_data = std::env::var("LOCALAPPDATA")
            .map_err(|_| anyhow::anyhow!("LOCALAPPDATA environment variable not found"))?;
        Ok(PathBuf::from(local_app_data).join("gresources"))
    }

    #[cfg(not(target_os = "windows"))]
    fn get_log_directory() -> Result<PathBuf> {
        let home = std::env::var("HOME")
            .map_err(|_| anyhow::anyhow!("HOME environment variable not found"))?;
        Ok(PathBuf::from(home).join(".local/share/gresources"))
    }
}
