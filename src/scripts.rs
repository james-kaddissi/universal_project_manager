use std::fs;
use std::path::Path;
use std::env;
use crate::util::get_install_path;

pub fn delete_script(script_name: &str) {
    let install_path = match get_install_path() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to get install path: {}", err);
            return;
        }
    };

    let scripts_dir = Path::new(&install_path).join("scripts");

    if !scripts_dir.exists() {
        eprintln!("Scripts directory not found: {}", scripts_dir.display());
        return;
    }

    for entry in fs::read_dir(&scripts_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(file_name) = path.file_stem() {
            if file_name == script_name {
                if let Err(err) = fs::remove_file(&path) {
                    eprintln!("Failed to delete script '{}': {}", script_name, err);
                    return;
                }

                println!("Script '{}' deleted successfully.", script_name);
                return;
            }
        }
    }

    println!("Script '{}' not found.", script_name);
}

pub fn add_script(script_name: &str) {
    let install_path = match get_install_path() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to get install path: {}", err);
            return;
        }
    };

    let scripts_dir = Path::new(&install_path).join("scripts");

    if !scripts_dir.exists() {
        eprintln!("Scripts directory not found: {}", scripts_dir.display());
        return;
    }

    for entry in fs::read_dir(&scripts_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(file_name) = path.file_stem() {
            if file_name == script_name {
                let file_extension = match path.extension() {
                    Some(ext) => ext.to_string_lossy().into_owned(),
                    None => {
                        eprintln!("Invalid script file: no extension found.");
                        return;
                    }
                };

                let dest_path = Path::new(&install_path).join(format!("{}.{}", script_name, file_extension));

                match fs::copy(&path, &dest_path) {
                    Ok(_) => {
                        println!("Script '{}' added successfully to '{}'.", script_name, dest_path.display());
                        return;
                    }
                    Err(err) => {
                        eprintln!("Failed to add script '{}': {}", script_name, err);
                        return;
                    }
                }
            }
        }
    }

    println!("Script '{}' not found.", script_name);
}

pub fn save_script(script_name: &str, script_path: Option<&str>) {
    let script_path = match script_path {
        Some(path) => path,
        None => {
            eprintln!("No script file path provided.");
            return;
        }
    };

    let install_path = match get_install_path() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to get install path: {}", err);
            return;
        }
    };

    let scripts_dir = Path::new(&install_path).join("scripts");
    if !scripts_dir.exists() {
        if let Err(err) = fs::create_dir_all(&scripts_dir) {
            eprintln!("Failed to create scripts directory: {}", err);
            return;
        }
    }

    let original_extension = match Path::new(script_path).extension() {
        Some(ext) => ext,
        None => {
            eprintln!("Invalid script file path: no extension found.");
            return;
        }
    };

    let new_script_path = scripts_dir.join(format!("{}.{}", script_name, original_extension.to_string_lossy()));

    if let Err(err) = fs::copy(script_path, &new_script_path) {
        eprintln!("Failed to save script: {}", err);
        return;
    }

    println!("Script '{}' saved successfully to '{}'.", script_name, new_script_path.display());
}
