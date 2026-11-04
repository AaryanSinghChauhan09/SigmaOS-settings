use sigmaos_settings::{Config, SettingsComponent, Component};
fn main() {
    env_logger::init();
    let mut c = SettingsComponent::new(Config::default());
    if let Err(e) = c.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS settings running");
    let _ = c.shutdown();
}
