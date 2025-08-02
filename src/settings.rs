use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub db_file_path: String,
    pub db_schema_path: String,
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn load() -> Result<Self> {
        let contents = fs::read_to_string("Settings.toml")?;
        let settings: Settings = toml::from_str(&contents)?;
        Ok(settings)
    }
}
