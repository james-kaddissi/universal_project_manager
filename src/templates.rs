use std::fs;
use std::io;
use std::path::Path;
use std::env;
use crate::project_init::init_project;
pub fn template_manager(action: &str, template_name: &str, project_name: Option<&str>, project_language: Option<&str>, project_main: Option<&str>) {
    match action {
        "save" => {
            // Get current directory path
            let current_dir = match env::current_dir() {
                Ok(path) => path,
                Err(err) => {
                    eprintln!("Failed to get current directory: {}", err);
                    return;
                }
            };

            // Create a templates directory if it doesn't exist
            let templates_dir = Path::new("J:\\universal_project_manager\\templates");
            if !templates_dir.exists() {
                if let Err(err) = fs::create_dir_all(&templates_dir) {
                    eprintln!("Failed to create templates directory: {}", err);
                    return;
                }
            }

            // Create a new directory for the template
            let template_path = templates_dir.join(template_name);
            if let Err(err) = fs::create_dir(&template_path) {
                eprintln!("Failed to create template directory: {}", err);
                return;
            }

            // Copy all contents from current directory to template directory recursively
            if let Err(err) = copy_dir_contents(&current_dir, &template_path) {
                eprintln!("Failed to copy directory contents: {}", err);
                return;
            }

            println!("Saved current directory as template '{}'", template_name);

        },
        "create" => {
            // Check if the project directory already exists
            

            // Define the path to the templates directory and the specific template
            let templates_dir = Path::new("J:\\universal_project_manager\\templates\\").join(template_name);

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
                    println!("Enter the projects main entry point (e.g., src/main.py, src/main.rs):");
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    input.trim().to_string()
                },
            };
            

            if let Err(err) = fs::create_dir(&dest_path) {
                eprintln!("Failed to create project directory: {}", err);
                return;
            }
            
            // Copy the template directory contents into the newly created project directory
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
            let templates_dir = Path::new("J:\\universal_project_manager\\templates"); 
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
        "list" => {
            let templates_dir = Path::new("J:\\universal_project_manager\\templates");
            if !templates_dir.exists() {
                eprintln!("No templates found.");
                return;
            }

            let entries = match fs::read_dir(templates_dir) {
                Ok(entries) => entries,
                Err(err) => {
                    eprintln!("Failed to read templates directory: {}", err);
                    return;
                }
            };

            for entry in entries {
                if let Ok(entry) = entry {
                    let template_name = entry.file_name();
                    println!("{}", template_name.to_string_lossy());
                }
            }
        },
        _ => {println!("Unsupported action '{}'.", action);}
    }
}

fn copy_dir_contents(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let file_name = src_path.file_name().ok_or(io::Error::new(io::ErrorKind::Other, "Invalid file name"))?;

        let dst_path = dst.join(file_name);

        if file_type.is_dir() {
            fs::create_dir_all(&dst_path)?;
            copy_dir_contents(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}