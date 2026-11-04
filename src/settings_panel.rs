#[derive(Debug, Clone, PartialEq)]
pub enum SettingsCategory { Display, Sound, Network, Power, Bluetooth, Appearance, Accessibility, Users, Security }

#[derive(Debug, Clone)]
pub struct DisplaySettings { pub brightness: u8, pub scale: f32, pub resolution: (u32, u32), pub dark_mode: bool }
impl Default for DisplaySettings {
    fn default() -> Self { Self { brightness: 80, scale: 1.0, resolution: (1920, 1080), dark_mode: false } }
}

#[derive(Debug, Clone)]
pub struct SoundSettings { pub volume: u8, pub muted: bool, pub output_device: String }
impl Default for SoundSettings {
    fn default() -> Self { Self { volume: 70, muted: false, output_device: "default".to_string() } }
}

pub struct SettingsManager {
    pub display: DisplaySettings,
    pub sound: SoundSettings,
}
impl SettingsManager {
    pub fn new() -> Self { Self { display: DisplaySettings::default(), sound: SoundSettings::default() } }
    pub fn set_brightness(&mut self, v: u8) { self.display.brightness = v.min(100); }
    pub fn set_volume(&mut self, v: u8) { self.sound.volume = v.min(100); }
    pub fn toggle_mute(&mut self) { self.sound.muted = !self.sound.muted; }
    pub fn toggle_dark_mode(&mut self) { self.display.dark_mode = !self.display.dark_mode; }
}
impl Default for SettingsManager { fn default() -> Self { Self::new() } }
