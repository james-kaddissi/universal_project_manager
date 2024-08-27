use serde::{Serialize, Deserialize};
use std::fs;
use regex::Regex;

#[cfg(unix)]
const CONFIG_PATH: &str = "/Users/james/WinDesktop/universal_project_manager/upmconfig.toml";

#[cfg(windows)]
const CONFIG_PATH: &str = "J:\\universal_project_manager\\upmconfig.toml";

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub default_flags: DefaultFlags,
    pub preferences: Preferences,
    pub warnings: Warnings,
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
}

#[derive(Deserialize, Serialize)]
pub struct Warnings {
    pub creation: bool,
    pub init: bool,
    pub run: bool,
    pub add: bool,
}

pub fn read_config_from() -> Config {
    let config_str = fs::read_to_string(CONFIG_PATH)
        .expect("Failed to read config file");
    toml::from_str(&config_str).expect("Failed to process config file")
}

fn write_config_to(config: &Config) {
    let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize to TOML");
    if let Err(e) = fs::write(CONFIG_PATH, toml_str) {
        eprintln!("Failed to write to config file: {}", e);
        return;
    }
}

pub fn set_license(argument: &str) {
    // Read the current configuration from upmconfig.toml
    let mut config = read_config_from();

    let mit_pattern = Regex::new(r#"(?i)mit"#).unwrap();
    let apache_pattern = Regex::new(r#"(?i)apache"#).unwrap();
    let gpl_v3_pattern = Regex::new(r#"(?i)gpl-?v?3(\.0)?"#).unwrap();
    let bsd_3_clause_pattern = Regex::new(r#"(?i)bsd-?3(\.0)?"#).unwrap();
    let agpl_v3_pattern = Regex::new(r#"(?i)agpl-?v?3(\.0)?"#).unwrap();
    let mpl_2_pattern = Regex::new(r#"(?i)mpl-?2(\.0)?"#).unwrap();
    let lgpl_v3_pattern = Regex::new(r#"(?i)lgpl-?v?3(\.0)?"#).unwrap();
    let epl_2_pattern = Regex::new(r#"(?i)epl-?2(\.0)?"#).unwrap();
    let unlicense_pattern = Regex::new(r#"(?i)unlicense"#).unwrap();
    let gpl_v2_pattern = Regex::new(r#"(?i)gpl-?v?2(\.0)?"#).unwrap();

    if mit_pattern.is_match(argument) {
        config.preferences.license = "MIT".to_string();
        println!("Default license updated to MIT");
    } else if apache_pattern.is_match(argument) {
        config.preferences.license = "Apache-2.0".to_string();
        println!("Default license updated to Apache License 2.0");
    } else if gpl_v3_pattern.is_match(argument) {
        config.preferences.license = "GPL-3.0".to_string();
        println!("Default license updated to GNU General Public License v3.0");
    } else if bsd_3_clause_pattern.is_match(argument) {
        config.preferences.license = "BSD-3-Clause".to_string();
        println!("Default license updated to BSD 3-Clause License");
    } else if agpl_v3_pattern.is_match(argument) {
        config.preferences.license = "AGPL-3.0".to_string();
        println!("Default license updated to GNU Affero General Public License v3.0");
    } else if mpl_2_pattern.is_match(argument) {
        config.preferences.license = "MPL-2.0".to_string();
        println!("Default license updated to Mozilla Public License 2.0");
    } else if lgpl_v3_pattern.is_match(argument) {
        config.preferences.license = "LGPL-3.0".to_string();
        println!("Default license updated to GNU Lesser General Public License v3.0");
    } else if epl_2_pattern.is_match(argument) {
        config.preferences.license = "EPL-2.0".to_string();
        println!("Default license updated to Eclipse Public License 2.0");
    } else if unlicense_pattern.is_match(argument) {
        config.preferences.license = "Unlicense".to_string();
        println!("Default license updated to Unlicense");
    } else if gpl_v2_pattern.is_match(argument) {
        config.preferences.license = "GPL-2.0".to_string();
        println!("Default license updated to GNU General Public License v2.0");
    } else {
        println!("The license '{}' is not recognized or supported by UPM.", argument);
        println!("To view a list of supported licenses, try 'upm list licenses'.");
        return; 
    }

    write_config_to(&config); 
    println!("License preference updated successfully.");
}

pub fn set_editor(argument: &str) {
    // Read the current configuration from upmconfig.toml
    let mut config = read_config_from();
    let vscode_pattern = Regex::new(r#"(?i)vs\s*code|visual\s*studio\s*code|visual[-\s]*"#).unwrap();
    let vim_pattern = Regex::new(r#"(?i)vim"#).unwrap();
    let eclipse_pattern = Regex::new(r#"(?i)eclipse"#).unwrap();
    let sublime_pattern = Regex::new(r#"(?i)sublime"#).unwrap();
    let atom_pattern = Regex::new(r#"(?i)atom"#).unwrap();
    let notepadpp_pattern = Regex::new(r#"(?i)notepad\+\+"#).unwrap();
    let gsedit_pattern = Regex::new(r#"(?i)gs\s*edit"#).unwrap();
    
    // Match input against regex patterns
    if vscode_pattern.is_match(argument) {
        config.preferences.editor = "vscode".to_string();
        println!("Default editor updated to VS Code");
    } else if vim_pattern.is_match(argument) {
        config.preferences.editor = "vim".to_string();
        println!("Default editor updated to Vim");
    } else if eclipse_pattern.is_match(argument) {
        config.preferences.editor = "eclipse".to_string();
        println!("Default editor updated to Eclipse");
    } else if sublime_pattern.is_match(argument) {
        config.preferences.editor = "sublime".to_string();
        println!("Default editor updated to Sublime Text");
    } else if atom_pattern.is_match(argument) {
        config.preferences.editor = "atom".to_string();
        println!("Default editor updated to Atom");
    } else if notepadpp_pattern.is_match(argument) {
        config.preferences.editor = "notepad++".to_string();
        println!("Default editor updated to Notepad++");
    } else if gsedit_pattern.is_match(argument) {
        config.preferences.editor = "gsedit".to_string();
        println!("Default editor updated to GS-Edit");
    }else {
        // Default case: set editor to the provided argument with a warning
        config.preferences.editor = argument.to_string();
        println!("Default editor updated to {}. This editor is not natively recognized by UPM.", argument);
        println!("Check for typos in your argument if you believe this is an error.");
        println!("To view a list of supported editors, try 'upm list editors'.");
    }
    write_config_to(&config);
    println!("Editor preference updated successfully.");
}

pub fn set_defaults(argument: &str) {
    println!("Setting defaults for '{}'", argument);

    // Read the current configuration from upmconfig.toml
    let mut config = read_config_from();

    // Determine which flag to toggle
    match argument {
        "git" => {
            config.default_flags.git = !config.default_flags.git;
            println!("git default flag updated to {}", config.default_flags.git);
        },
        "ignore" => {
            config.default_flags.ignore = !config.default_flags.ignore;
            println!("ignore default flag updated to {}", config.default_flags.ignore);
        },
        _ => {
            println!("Unsupported argument '{}'. Use 'git' or 'ignore'.", argument);
            return;
        }
    }

    // Save the updated configuration back to the file
    write_config_to(&config);
    println!("Defaults updated successfully.");
}
