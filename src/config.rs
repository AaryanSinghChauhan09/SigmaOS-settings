use serde::{Deserialize, Serialize};
use crate::error::{ComponentError, Result};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config { pub enabled: bool, pub log_level: String, pub dark_theme: bool }
impl Default for Config {
    fn default() -> Self { Self { enabled: true, log_level: "info".to_string(), dark_theme: false } }
}
impl Config {
    pub fn validate(&self) -> Result<()> {
        if self.log_level.is_empty() { return Err(ComponentError::Config("log_level cannot be empty".to_string())); }
        Ok(())
    }
    pub fn from_file(_path: &str) -> Result<Self> { Ok(Self::default()) }
}
