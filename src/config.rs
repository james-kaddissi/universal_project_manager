use serde::{Serialize, Deserialize};
use std::fs;

#[cfg(unix)]
const CONFIG_PATH: &str = "/Users/james/WinDesktop/universal_project_manager/upmconfig.toml";

#[cfg(windows)]
const CONFIG_PATH: &str = "J:\\universal_project_manager\\upmconfig.toml";

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub default_flags: DefaultFlags,
    pub preferences: Preferences,
}

#[derive(Deserialize, Serialize)]
pub struct DefaultFlags {
    pub git: bool,
    pub ignore: bool,
    pub venv: bool,
    pub license: bool,
    pub readme: bool,
    pub tests: bool,
    pub docs: bool,
    pub docker: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Preferences {
    pub editor: String,
    pub license: String,
    pub open_editor_on_create: bool,
}

pub fn read_config_from() -> Config {
    let config_str = fs::read_to_string(CONFIG_PATH)
        .expect("Failed to read config file");
    toml::from_str(&config_str).expect("Failed to process config file")
}

pub fn write_config_to(config: &Config) {
    let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize to TOML");
    if let Err(e) = fs::write(CONFIG_PATH, toml_str) {
        eprintln!("Failed to write to config file: {}", e);
        return;
    }
}