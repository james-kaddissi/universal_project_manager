use std::fs;
use std::io;
use std::path::Path;
use std::env;
use crate::project_init::init_project;
use crate::util::get_install_path;

pub fn template_manager(action: &str, template_name: &str, project_name: Option<&str>, project_language: Option<&str>, project_main: Option<&str>) {
    match action {
        "save" => {
            let install_path = match get_install_path() {
                Ok(path) => path,
                Err(err) => {
                    eprintln!("Failed to get install path: {}", err);
                    return;
                }
            };

            let templates_dir = Path::new(&install_path).join("templates");
            if !templates_dir.exists() {
                if let Err(err) = fs::create_dir_all(&templates_dir) {
                    eprintln!("Failed to create templates directory: {}", err);
                    return;
                }
            }

            let template_path = templates_dir.join(template_name);
            if let Err(err) = fs::create_dir(&template_path) {
                eprintln!("Failed to create template directory: {}", err);
                return;
            }

            let current_dir = match env::current_dir() {
                Ok(path) => path,
                Err(err) => {
                    eprintln!("Failed to get current directory: {}", err);
                    return;
                }
            };

            if let Err(err) = copy_dir_contents(&current_dir, &template_path) {
                eprintln!("Failed to copy directory contents: {}", err);
                return;
            }

            println!("Saved current directory as template '{}'", template_name);

        },
        "create" => {
            let install_path = match get_install_path() {
                Ok(path) => path,
                Err(err) => {
                    eprintln!("Failed to get install path: {}", err);
                    return;
                }
            };

            let templates_dir = Path::new(&install_path).join("templates").join(template_name);

            if !templates_dir.exists() {
                eprintln!("Template '{}' does not exist.", template_name);
                return;
            }

            let project_name = match project_name {
                Some(name) => name.to_string(),
                None => {
                    let mut input = String::new();
                    println!("Enter the project name: ");
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    input.trim().to_string()
                },
            };

            let dest_path = env::current_dir().unwrap().join(&project_name);
            if dest_path.exists() {
                eprintln!("Destination directory '{}' already exists.", dest_path.display());
                return;
            }

            let project_language = match project_language {
                Some(lang) => lang.to_string(),
                None => {
                    let mut input = String::new();
                    println!("Enter the project language (e.g., python, rust, cpp):");
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    input.trim().to_string()
                },
            };

            let project_main = match project_main {
                Some(main) => main.to_string(),
                None => {
                    let mut input = String::new();
                    println!("Enter the project's main entry point (e.g., src/main.py, src/main.rs):");
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    input.trim().to_string()
                },
            };

            if let Err(err) = fs::create_dir(&dest_path) {
                eprintln!("Failed to create project directory: {}", err);
                return;
            }
            
            if let Err(err) = copy_dir_contents(&templates_dir, &dest_path) {
                eprintln!("Failed to copy template directory: {}", err);
                return;
            }

            if let Err(err) = env::set_current_dir(&dest_path) {
                eprintln!("Failed to navigate into project directory: {}", err);
                return;
            }

            init_project(Some(&project_language), Some(&project_main));
            println!("Created project '{}' from template '{}'", project_name, template_name);
        },
        "delete" => {
            let install_path = match get_install_path() {
                Ok(path) => path,
                Err(err) => {
                    eprintln!("Failed to get install path: {}", err);
                    return;
                }
            };

            let templates_dir = Path::new(&install_path).join("templates");
            let template_path = templates_dir.join(template_name);

            if template_path.exists() {
                if let Err(err) = fs::remove_dir_all(&template_path) {
                    eprintln!("Failed to delete template '{}': {}", template_name, err);
                } else {
                    println!("Template '{}' deleted successfully.", template_name);
                }
            } else {
                eprintln!("Template '{}' not found.", template_name);
            }
        },
        _ => eprintln!("Unknown action: {}", action),
    }
}

fn copy_dir_contents(src: &Path, dst: &Path) -> io::Result<()> {
    if !src.exists() || !src.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Source directory not found"));
    }

    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if entry_path.is_dir() {
            copy_dir_contents(&entry_path, &dst_path)?;
        } else {
            fs::copy(entry_path, dst_path)?;
        }
    }

    Ok(())
}