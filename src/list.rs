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
        "languages" => {
            println!(" ");
            println!("Supported languages:");
            println!(" ");
            println!("1. Python");
            println!("2. C");
            println!("3. C++");
            println!("4. Java");
            println!("5. JavaScript");
            println!("6. TypeScript");
            println!("7. Rust");
            println!("8. Go");
            println!("9. Ruby");
            println!("10. Swift");
            println!("11. Kotlin");
            println!("12. Dart");
            println!("13. PHP");
            println!("14. HTML");
            println!("15. SQL");
            println!("16. Shell");
            println!("17. C#");
            println!("18. R");
            println!("19. Scala");
            println!("20. Perl");
            println!("21. Lua");
            println!("22. Groovy");
            println!("23. React");
            println!("24. Haskell");
            println!("25. Erlang");
            println!("26. COBOL");
            println!("27. Fortran");
            println!("28. Lisp");
            println!("29. MATLAB");
            println!("30. Obj-C");
            println!("31. Pascal");
            println!(" ");
        },
        _ => {println!("Unsupported argument '{}'.", argument);}
    }
}


