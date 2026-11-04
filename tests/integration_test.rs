use sigmaos_settings::{Config, SettingsComponent, SettingsManager, Component};

#[test]
fn test_settings_brightness_clamp_aaa() {
    let mut mgr = SettingsManager::new();
    mgr.set_brightness(150);
    assert_eq!(mgr.display.brightness, 100);
}

#[test]
fn test_settings_toggle_mute_aaa() {
    let mut mgr = SettingsManager::new();
    assert!(!mgr.sound.muted);
    mgr.toggle_mute();
    assert!(mgr.sound.muted);
    mgr.toggle_mute();
    assert!(!mgr.sound.muted);
}

#[test]
fn test_settings_dark_mode_aaa() {
    let mut mgr = SettingsManager::new();
    assert!(!mgr.display.dark_mode);
    mgr.toggle_dark_mode();
    assert!(mgr.display.dark_mode);
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut c = SettingsComponent::new(Config::default());
    assert!(c.initialize().is_ok());
    assert!(c.health_check().is_ok());
    assert!(c.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let c = SettingsComponent::new(Config::default());
    assert_eq!(c.name(), "sigmaos-settings");
}
