use crate::config::read_config_from;
use crate::project_database::load_projects_db;
use std::fs;
use std::path::Path;
pub fn list_manager(argument: &str) {
    match argument {
        "editors" => {
            println!("Supported editors:");
            println!("1. VS Code");
            println!("2. Vim");
            println!("3. Eclipse");
            println!("4. Sublime Text");
            println!("5. Atom");
            println!("7. Notepad++");
            println!("8. GS-Edit");
        },
        "templates" => {
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
        "licenses" => {
            let license_dir = Path::new("J:\\universal_project_manager\\licenses");
            if !license_dir.exists() {
                eprintln!("No licenses found.");
                return;
            }

            let entries = match fs::read_dir(license_dir) {
                Ok(entries) => entries,
                Err(err) => {
                    eprintln!("Failed to read licenses directory: {}", err);
                    return;
                }
            };
            for entry in entries {
                if let Ok(entry) = entry {
                    let license_name = entry.file_name();
                    println!("{}", license_name.to_string_lossy());
                }
            }
        },
        "projects" => {
            let db = load_projects_db();
            for (project_name, info) in db.projects.iter() {
                println!("{}: {}", project_name, info.project_path);
            }
        },
        "preferences" => {
            let config = read_config_from();
            println!(" ");
            println!("Git Flag: {}", config.default_flags.git);
            println!("Ignore Flag: {}", config.default_flags.ignore);
            println!("Venv Flag: {}", config.default_flags.venv);
            println!("License Flag: {}", config.default_flags.license);
            println!("Readme Flag: {}", config.default_flags.readme);
            println!("Tests Flag: {}", config.default_flags.tests);
            println!("Docs Flag: {}", config.default_flags.docs);
            println!("Docker Flag: {}", config.default_flags.docker);
            println!(" ");
            println!("Editor: {}", config.preferences.editor);
            println!("License: {}", config.preferences.license);
            println!(" ");
            println!("Use 'upm config defaults <flag_to_toggle>' or 'upm config editor/license <editor/license_name>' to modify preferences.");
            println!(" ");
        },
        _ => {println!("Unsupported argument '{}'.", argument);}
    }
}


