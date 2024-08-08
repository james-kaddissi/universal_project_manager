use std::process::Command;
use std::fs;
use std::path::PathBuf;
use crate::load_projects_db;
use crate::clean_path;
use std::env;
use std::io::Write;

enum PackageManager {
    Pip,
    Cargo,
    Npm,
    Gem,
    Other(String), // For package managers that are just a single command
}

pub fn add_package(package_name: &str) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let current_dir_str = clean_path(&current_dir);

    let db = load_projects_db();
    
    let project_info = db.projects.iter().find(|(_key, value)| {
        current_dir_str.starts_with(&value.project_path)
    });

    if let Some((_project_name, info)) = project_info {
        let package_manager = match info.project_language.as_str() {
            "python" => PackageManager::Pip,
            "rust" => PackageManager::Cargo,
            "javascript" => PackageManager::Npm,
            "ruby" => PackageManager::Gem,
            "java" => PackageManager::Other("gradle".to_string()),
            "c#" => PackageManager::Other("nuget".to_string()),
            _ => {
                println!("Package management not supported for {}", info.project_language);
                return;
            },
        };

        execute_package_command(package_manager, &PathBuf::from(&info.project_path), package_name);
    } else {
        println!("Current directory is not a recognized UPM project.");
    }
}

fn execute_package_command(package_manager: PackageManager, current_dir: &PathBuf, package_name: &str) {
    match package_manager {
        PackageManager::Pip => {
            let requirements_path = current_dir.join("requirements.txt");
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(&requirements_path)
                .expect("Failed to open requirements.txt");
            writeln!(file, "{}", package_name).expect("Failed to write to requirements.txt");

            let pip_path = if cfg!(target_os = "windows") {
                current_dir.join("venv").join("Scripts").join("pip.exe")
            } else {
                current_dir.join("venv").join("bin").join("pip")
            };

            let status = Command::new(pip_path)
                .args(&["install", package_name])
                .status()
                .expect("Failed to install package");
            if !status.success() {
                eprintln!("Failed to add package '{}'.", package_name);
            }
        },
        PackageManager::Cargo => {
            let status = Command::new("cargo")
                .args(&["add", package_name])
                .current_dir(current_dir)
                .status()
                .expect("Failed to run cargo command");
            if !status.success() {
                eprintln!("Failed to add package '{}'.", package_name);
            }
        },
        PackageManager::Npm => {
            let status = Command::new("npm")
                .args(&["install", "--save", package_name])
                .current_dir(current_dir)
                .status()
                .expect("Failed to run npm command");
            if !status.success() {
                eprintln!("Failed to add package '{}'.", package_name);
            }
        },
        PackageManager::Gem => {
            let status = Command::new("gem")
                .args(&["install", package_name])
                .current_dir(current_dir)
                .status()
                .expect("Failed to run gem command");
            if !status.success() {
                eprintln!("Failed to add package '{}'.", package_name);
            }
        },
        PackageManager::Other(cmd) => match cmd.as_str() {
            "gradle" => {
                let status = Command::new(cmd)
                    .args(&["install", package_name])
                    .current_dir(current_dir)
                    .status()
                    .expect("Failed to run package manager command");
                if !status.success() {
                    eprintln!("Failed to add package '{}'.", package_name);
                }
            },
            "nuget" => {
                let status = Command::new(cmd)
                    .args(&["install", package_name])
                    .current_dir(current_dir)
                    .status()
                    .expect("Failed to run package manager command");
                if !status.success() {
                    eprintln!("Failed to add package '{}'.", package_name);
                }
            },
            _ => {
                eprintln!("Package management not supported for '{}'.", cmd);
            }
        },
    }

    println!("Package '{}' added successfully.", package_name);
}