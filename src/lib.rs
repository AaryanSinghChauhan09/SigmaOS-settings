//! SigmaOS-settings - SigmaOS Component
//!
//! Critical system infrastructure component for SigmaOS,
//! inspired by Arch Linux and Debian architecture.

use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Component configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub enabled: bool,
    pub debug: bool,
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            debug: false,
            log_level: "info".to_string(),
        }
    }
}

/// Component main trait
pub trait Component {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> Result<()>;
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn status(&self) -> ComponentStatus;
}

/// Component status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentStatus {
    Running,
    Stopped,
    Error(String),
    Initializing,
}

/// Main component implementation
pub struct Settings {
    config: Config,
    status: ComponentStatus,
}

pub type settings = Settings;

impl Settings {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            status: ComponentStatus::Stopped,
        }
    }

    pub fn from_config_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(Self::new(config))
    }

    pub fn name(&self) -> &str {
        "SigmaOS-settings"
    }

    pub fn version(&self) -> &str {
        "0.1.0"
    }

    pub fn initialize(&mut self) -> Result<()> {
        info!("Initializing SigmaOS-settings");
        self.status = ComponentStatus::Initializing;
        
        // Initialize logging
        if self.config.debug {
            let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
                .try_init();
        } else {
            let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&self.config.log_level))
                .try_init();
        }
        
        info!("Component configuration: {:?}", self.config);
        self.status = ComponentStatus::Stopped;
        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        info!("Starting SigmaOS-settings");
        self.status = ComponentStatus::Running;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        info!("Stopping SigmaOS-settings");
        self.status = ComponentStatus::Stopped;
        Ok(())
    }

    pub fn status(&self) -> ComponentStatus {
        self.status.clone()
    }
}

impl Component for Settings {
    fn name(&self) -> &str {
        Settings::name(self)
    }

    fn version(&self) -> &str {
        Settings::version(self)
    }

    fn initialize(&mut self) -> Result<()> {
        Settings::initialize(self)
    }

    fn start(&mut self) -> Result<()> {
        Settings::start(self)
    }

    fn stop(&mut self) -> Result<()> {
        Settings::stop(self)
    }

    fn status(&self) -> ComponentStatus {
        Settings::status(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let config = Config::default();
        let component = Settings::new(config);
        assert_eq!(component.name(), "SigmaOS-settings");
        assert_eq!(component.version(), "0.1.0");
    }

    #[test]
    fn test_initialization() {
        let mut component = Settings::new(Config::default());
        component.initialize().unwrap();
        assert!(matches!(component.status(), ComponentStatus::Stopped));
    }

    #[test]
    fn test_start_stop() {
        let mut component = Settings::new(Config::default());
        component.initialize().unwrap();
        component.start().unwrap();
        assert!(matches!(component.status(), ComponentStatus::Running));
        component.stop().unwrap();
        assert!(matches!(component.status(), ComponentStatus::Stopped));
    }
}
