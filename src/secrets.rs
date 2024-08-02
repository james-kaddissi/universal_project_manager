use std::io::{BufRead, BufReader, BufWriter, Write};
use std::fs::{self, OpenOptions};
use std::env;

use crate::util::clean_path;
use crate::project_database::load_projects_db;

fn load_secrets(file_path: &std::path::Path) -> std::collections::HashMap<String, String> {
    let mut secrets_map = std::collections::HashMap::new();

    if let Ok(file) = fs::File::open(file_path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some((key, value)) = parse_env_line(&line) {
                    secrets_map.insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    secrets_map
}

fn save_secrets(file_path: &std::path::Path, secrets_map: &std::collections::HashMap<String, String>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    let mut writer = BufWriter::new(file);

    for (key, value) in secrets_map {
        writeln!(writer, "{}={}", key, value).expect("Failed to write to .env file");
    }
}

fn parse_env_line(line: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() == 2 {
        Some((parts[0].trim(), parts[1].trim()))
    } else {
        None
    }
}

pub fn secrets_manager(action: &str, secret: &str, secret_value: &str) {
    let current_dir = env::current_dir().unwrap();
    let current_dir_str = clean_path(&current_dir);
    let db = load_projects_db();
    if db.projects.iter().any(|(_key, value)| current_dir_str.starts_with(&value.project_path)) {
        if action == "save" || action == "add" {
            let current_dir = env::current_dir().unwrap();
            let env_file_path = current_dir.join(".env");
            let mut secrets_map = load_secrets(&env_file_path);
            secrets_map.insert(secret.to_string(), secret_value.to_string());
            save_secrets(&env_file_path, &secrets_map);
            println!("Secret added successfully.");
        } else if action == "delete" || action == "remove" {
            let current_dir = env::current_dir().unwrap();
            let env_file_path = current_dir.join(".env");

            let mut secrets_map = load_secrets(&env_file_path);

            if secrets_map.remove(secret).is_some() {
                save_secrets(&env_file_path, &secrets_map);
                println!("Secret removed successfully.");
            } else {
                println!("Secret not found.");
            }
        } else if action == "show" {
            let current_dir = env::current_dir().unwrap();
            let env_file_path = current_dir.join(".env");

            let secrets_map = load_secrets(&env_file_path);

            for (key, value) in secrets_map {
                if key == secret {
                    println!("{}={}", key, value);
                    return;
                }
            }
        } else {
            println!("Unsupported action '{}'.", action);
        }
    } else {
        println!("This directory is not recognized as a UPM project.");
        return;
    }
}
