use anyhow::Result;
use settings::{ComponentStatus, Config, Settings};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_lifecycle_aaa() -> Result<()> {
        let config = Config::default();
        let mut s = Settings::new(config);
        
        assert_eq!(s.status(), ComponentStatus::Stopped);
        s.initialize()?;
        s.start()?;
        assert_eq!(s.status(), ComponentStatus::Running);
        s.stop()?;
        assert_eq!(s.status(), ComponentStatus::Stopped);
        Ok(())
    }

    #[test]
    fn test_settings_metadata_aaa() {
        let config = Config::default();
        let s = Settings::new(config);
        assert_eq!(s.name(), "SigmaOS-settings");
        assert_eq!(s.version(), "0.1.0");
    }
}
