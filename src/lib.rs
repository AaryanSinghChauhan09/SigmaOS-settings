pub mod config;
pub mod error;
pub mod service;
pub mod settings_panel;
pub use config::Config;
pub use error::{ComponentError, Result};
pub use service::{Service, ServiceStatus};
pub use settings_panel::{DisplaySettings, SettingsCategory, SettingsManager, SoundSettings};
pub trait Component {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> error::Result<()>;
    fn shutdown(&mut self) -> error::Result<()>;
    fn health_check(&self) -> error::Result<()>;
}
pub struct SettingsComponent { service: service::Service }
impl SettingsComponent {
    pub fn new(config: Config) -> Self { Self { service: service::Service::new(config) } }
}
impl Component for SettingsComponent {
    fn name(&self) -> &str { "sigmaos-settings" }
    fn version(&self) -> &str { env!("CARGO_PKG_VERSION") }
    fn initialize(&mut self) -> error::Result<()> { self.service.start() }
    fn shutdown(&mut self) -> error::Result<()> { self.service.stop() }
    fn health_check(&self) -> error::Result<()> {
        if self.service.is_running() { Ok(()) }
        else { Err(error::ComponentError::Service("settings not running".to_string())) }
    }
}
pub mod network;
pub mod display;
pub mod sound;

/// Settings subsystem error type.
#[derive(Debug)]
pub enum SettingsError {
    ValidationError(String),
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidationError(m) => write!(f, "validation error: {m}"),
        }
    }
}

impl std::error::Error for SettingsError {}

/// Key/value store backing all settings panels.
#[derive(Debug, Default)]
pub struct SettingsStore {
    values: std::collections::HashMap<String, String>,
}

impl SettingsStore {
    pub fn new() -> Self { Self::default() }
    pub fn set(&mut self, key: &str, value: String) {
        self.values.insert(key.to_string(), value);
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }
}

/// Contract implemented by every settings panel.
pub trait SettingsPanel {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
    fn apply(&self, store: &mut SettingsStore) -> std::result::Result<(), SettingsError>;
}
