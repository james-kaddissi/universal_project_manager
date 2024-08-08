use std::fs;
use std::path::Path;
use std::env;

pub fn delete_script(script_name: &str) {
    // Get current directory
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to get current directory: {}", err);
            return;
        }
    };

    // Locate scripts directory
    let scripts_dir = current_dir.join("J:\\universal_project_manager\\scripts");

    // Iterate over files in scripts directory
    for entry in fs::read_dir(&scripts_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // Check if the file matches the script_name (without extension)
        if let Some(file_name) = path.file_stem() {
            if file_name == script_name {
                // Delete the file
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
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to get current directory: {}", err);
            return;
        }
    };

    // Locate scripts directory
    let scripts_dir = current_dir.join("J:\\universal_project_manager\\scripts");

    // Iterate over files in scripts directory
    for entry in fs::read_dir(&scripts_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // Check if the file matches the script_name (without extension)
        if let Some(file_name) = path.file_stem() {
            if file_name == script_name {
                // Determine the extension of the file
                let file_extension = match path.extension() {
                    Some(ext) => ext.to_string_lossy().into_owned(),
                    None => {
                        eprintln!("Invalid script file: no extension found.");
                        return;
                    }
                };

                // Construct the destination path in the current directory
                let dest_path = current_dir.join(format!("{}.{}", script_name, file_extension));

                // Copy the script file to the current directory
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
    // Ensure script_path is provided
    let script_path = match script_path {
        Some(path) => path,
        None => {
            eprintln!("No script file path provided.");
            return;
        }
    };

    // Get current directory
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to get current directory: {}", err);
            return;
        }
    };

    // Create a scripts directory if it doesn't exist
    let scripts_dir = current_dir.join("J:\\universal_project_manager\\scripts");
    if !scripts_dir.exists() {
        if let Err(err) = fs::create_dir_all(&scripts_dir) {
            eprintln!("Failed to create scripts directory: {}", err);
            return;
        }
    }

    // Determine the file extension from script_path
    let original_extension = match Path::new(script_path).extension() {
        Some(ext) => ext,
        None => {
            eprintln!("Invalid script file path: no extension found.");
            return;
        }
    };

    // Create the full path for the new script file
    let new_script_path = scripts_dir.join(format!("{}.{}", script_name, original_extension.to_string_lossy()));

    // Copy the script file to the scripts directory
    if let Err(err) = fs::copy(script_path, &new_script_path) {
        eprintln!("Failed to save script: {}", err);
        return;
    }

    println!("Script '{}' saved successfully at '{}'.", script_name, new_script_path.display());
}
