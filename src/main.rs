use clap::{Arg, Command as ClapCommand };
use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};
use regex::Regex;
use std::fs::{self};
use std::io::{self, Write};
pub mod project_init;
pub mod project_database;
pub mod secrets;
pub mod util;
pub mod config;

enum PackageManager {
    Pip,
    Cargo,
    Npm,
    Gem,
    Other(String), // For package managers that are just a single command
}

use crate::project_init::{create_project};
use crate::util::{clean_path};
use crate::project_database::{load_projects_db, add_project_to_db, save_projects_db};
use crate::config::{read_config_from, write_config_to};
use crate::secrets::{secrets_manager};


fn main() {
    let config = read_config_from();
    let matches = ClapCommand::new("upm")
        .version("0.1.2")
        .about("Manages programming projects")
        .subcommand(
            ClapCommand::new("new")
                .about("Creates a new project")
                .arg(Arg::new("PROJECT_NAME")
                    .help("The name of the project")
                    .required(true)
                    .index(1))
                .arg(Arg::new("LANGUAGE")
                    .help("Specifies the language of the project")
                    .required(true)
                    .index(2))
                .arg(Arg::new("git")
                    .long("git")
                    .help("Initializes the project with git")
                    .action(clap::ArgAction::SetTrue)) 
                .arg(Arg::new("ignore")
                    .long("ignore")
                    .help("Initializes a .gitignore")
                    .action(clap::ArgAction::SetTrue)
                    .requires("git")) // Makes "ignore" require "git"
                .arg(Arg::new("venv")
                    .long("venv")
                    .help("Initializes a venv")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("license")
                    .long("license")
                    .help("Initializes a license. Uses default license if no argument is provided.")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("readme")
                    .long("readme")
                    .help("Initializes a readme. Uses default readme.")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("tests")
                    .long("tests")
                    .help("Initializes a tests directory.")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("docs")
                    .long("docs")
                    .help("Initializes a docs directory.")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("docker")
                    .long("docker")
                    .help("Initializes the project with docker")
                    .action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            ClapCommand::new("add")
                .about("Adds a package to the project")
                .arg(Arg::new("PACKAGE_NAME")
                    .help("The name of the package to add")
                    .required(true)
                    .index(1)),
        )
        .subcommand(
            ClapCommand::new("run")
                .about("Runs the main entrypoint of the project")
        )
        .subcommand(
            ClapCommand::new("init")
                .about("Initializes the current directory as a upm project")
                .arg(Arg::new("LANGUAGE")
                    .help("Specifies the language of the project")
                    .required(false)
                    .index(1))
                .arg(Arg::new("MAIN")
                    .help("Specifies the main entry point of the project")
                    .required(false)
                    .index(2))
        )
        .subcommand(
            ClapCommand::new("config")
                .about("Allows you to make modifications to the upm project settings")
                .arg(Arg::new("MODIFIER")
                    .help("Specifies the desired modifier of the project")
                    .required(true)
                    .index(1))
                .arg(Arg::new("ARGUMENT")
                    .help("pass the desired arguments")
                    .required(true)
                    .index(2))
        )
        .subcommand(
            ClapCommand::new("help")
                .about("Prints help information")
                .arg(Arg::new("SUBCOMMAND")
                    .help("The subcommand to get help for")
                    .required(false)
                    .index(1))
        )
        .subcommand(
            ClapCommand::new("template")
                .about("Contains subcommands for managing templates")
                .arg(Arg::new("ACTION")
                    .help("The action to perform on the template")
                    .required(true)
                    .index(1))
                .arg(Arg::new("TEMPLATE_NAME")
                    .help("The name of the template")
                    .required(false)
                    .index(2))
                .arg(Arg::new("PROJECT_NAME")
                    .help("The name of the project")
                    .required(false)
                    .index(3))
                .arg(Arg::new("LANGUAGE")
                    .help("Specifies the language of the project")
                    .required(false)
                    .index(4))
                .arg(Arg::new("MAIN")
                    .help("Specifies the main entry point of the project")
                    .required(false)
                    .index(5))
        )
        .subcommand(
            ClapCommand::new("list")
                .about("Lists details about UPM.")
                .arg(Arg::new("ARGUMENT")
                    .help("The argument to view. Try 'editors' or 'templates'.")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            ClapCommand::new("secrets")
                .about("Manages secrets for the project")
                .arg(Arg::new("ACTION")
                    .help("The action to perform on the secrets")
                    .required(true)
                    .index(1))
                .arg(Arg::new("SECRET")
                    .help("The secret to act on")
                    .required(true)
                    .index(2))
                .arg(Arg::new("SECRET_VALUE")
                    .help("The secret value")
                    .required(false)
                    .index(3))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub_m)) => {
            let project_name = sub_m.get_one::<String>("PROJECT_NAME").unwrap();
            let project_language = sub_m.get_one::<String>("LANGUAGE").unwrap();
        
            let git = sub_m.contains_id("git") || config.default_flags.git;
            let ignore = sub_m.contains_id("ignore") || (config.default_flags.ignore && git);
            let venv = sub_m.contains_id("venv") || config.default_flags.venv;
            let license = sub_m.contains_id("license") || config.default_flags.license;
            let readme = sub_m.contains_id("readme") || config.default_flags.readme;
            let tests = sub_m.contains_id("tests") || config.default_flags.tests;
            let docs = sub_m.contains_id("docs") || config.default_flags.docs;
            let docker = sub_m.contains_id("docker") || config.default_flags.docker;
        
            create_project(project_name, project_language, git, ignore, venv, license, readme, tests, docs, docker);
        },
        Some(("add", sub_m)) => {
            let package_name = sub_m.get_one::<String>("PACKAGE_NAME").unwrap();
            add_package(package_name);
        },
        Some(("run", _)) => {
            run_project();
        },
        Some(("init", sub_m)) => {
            let project_language = sub_m.get_one::<String>("LANGUAGE");
            let project_main = sub_m.get_one::<String>("MAIN");
            init_project(project_language.map(String::as_str), project_main.map(String::as_str));
        },
        Some(("config", sub_m)) => {
            let modifier = sub_m.get_one::<String>("MODIFIER").unwrap();
            let argument = sub_m.get_one::<String>("ARGUMENT").unwrap();

            if modifier == "main" {
                set_main_path(argument);
            }
            if modifier == "defaults" {
                set_defaults(argument);
            }
            if modifier == "editor" {
                set_editor(argument);
            }
            if modifier == "license" {
                set_license(argument);
            }
        },
        Some(("template", sub_m)) => {
            let action = sub_m.get_one::<String>("ACTION").unwrap();
            let template_name = sub_m
                .get_one::<String>("TEMPLATE_NAME")
                .map(String::to_string)
                .unwrap_or_default();
            let project_name = sub_m.get_one::<String>("PROJECT_NAME");
            let project_language = sub_m.get_one::<String>("LANGUAGE");
            let project_main = sub_m.get_one::<String>("MAIN");
            template_manager(action, &template_name, project_name.map(String::as_str), project_language.map(String::as_str), project_main.map(String::as_str));
        }
        Some(("list", sub_m)) => {
            let argument = sub_m.get_one::<String>("ARGUMENT").unwrap();
            list_manager(argument);
        },
        Some(("secrets", sub_m)) => {
            let action = sub_m.get_one::<String>("ACTION").unwrap();
            let secret = sub_m.get_one::<String>("SECRET").unwrap();
            let secret_value = sub_m.get_one::<String>("SECRET_VALUE").map(String::to_string).unwrap_or_default();

            secrets_manager(action, secret, &secret_value);
        },
        _ => {}
    }
}


fn list_manager(argument: &str) {
    match argument {
        "editors" => {
            println!("Supported editors:");
            println!("1. VS Code");
            println!("2. Vim");
            println!("3. Eclipse");
            println!("4. Sublime Text");
            println!("5. Atom");
            println!("6. Notepad");
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
        _ => {println!("Unsupported argument '{}'.", argument);}
    }
}

fn template_manager(action: &str, template_name: &str, project_name: Option<&str>, project_language: Option<&str>, project_main: Option<&str>) {
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

fn set_license(argument: &str) {
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

fn set_editor(argument: &str) {
    // Read the current configuration from upmconfig.toml
    let mut config = read_config_from();
    let vscode_pattern = Regex::new(r#"(?i)vs\s*code|visual\s*studio\s*code|visual[-\s]*"#).unwrap();
    let vim_pattern = Regex::new(r#"(?i)vim"#).unwrap();
    let eclipse_pattern = Regex::new(r#"(?i)eclipse"#).unwrap();
    let sublime_pattern = Regex::new(r#"(?i)sublime"#).unwrap();
    let atom_pattern = Regex::new(r#"(?i)atom"#).unwrap();
    let notepad_pattern = Regex::new(r#"(?i)notepad"#).unwrap();
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
    } else if notepad_pattern.is_match(argument) {
        config.preferences.editor = "notepad".to_string();
        println!("Default editor updated to Notepad");
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

fn set_defaults(argument: &str) {
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


fn init_project(project_language: Option<&str>, project_main: Option<&str>) {
    let current_dir = env::current_dir().unwrap();
    let current_dir_str = clean_path(&current_dir);
    let db = load_projects_db();

    if db.projects.iter().any(|(_key, value)| current_dir_str.starts_with(&value.project_path)) {
        println!("This directory is already recognized as a UPM project.");
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
            loop {
                println!("Enter the relative path to the main file to run (e.g., src/main.py):");
                io::stdout().flush().expect("Failed to flush stdout");
                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let input = input.trim();
                let main_file_path = current_dir.join(input);

                if main_file_path.exists() {
                    break input.to_string();
                } else {
                    println!("The file '{}' does not exist. Please enter a valid path.", input);
                }
            }
        },
    };

    add_project_to_db(current_dir.file_name().unwrap().to_str().unwrap(), &current_dir_str, &project_language, &project_main);
    println!("Initialized '{}' as a UPM project with language '{}' and main file '{}'.", current_dir.file_name().unwrap().to_str().unwrap(), project_language, project_main);
}

fn run_project() {
    let current_dir = env::current_dir().unwrap();
    let current_dir_str = clean_path(&current_dir);
    
    let db = load_projects_db();
    
    let project_info = db.projects.iter().find(|(_key, value)| {
        current_dir_str.starts_with(&value.project_path)
    });

    if let Some((_project_name, info)) = project_info {
        // Construct the path to the project's main file
        let script_path = Path::new(&info.project_path).join(&info.project_main);
        let script_path_str = script_path.to_str().unwrap();

        match info.project_language.as_str() {
            "python" => {
                if let Err(e) = Command::new("python3").arg(script_path_str).status() {
                    eprintln!("Failed to execute project: {}", e);
                }
            },
            "rust" => {
                if let Err(e) = Command::new("cargo").arg("run").current_dir(&info.project_path).status() {
                    eprintln!("Failed to execute project: {}", e);
                }
            },
            "cpp" | "c++" => {
                let compile_status = if cfg!(target_os = "windows") {
                    Command::new("g++").args([&info.project_main, "-o", "a.exe"]).status()
                } else {
                    Command::new("g++").args([&info.project_main, "-o", "a.out"]).status()
                };

                if let Ok(status) = compile_status {
                    if status.success() {
                        let run_status = if cfg!(target_os = "windows") {
                            Command::new("./a.exe").status()
                        } else {
                            Command::new("./a.out").status()
                        };
                        
                        if let Err(e) = run_status {
                            eprintln!("Failed to run compiled program: {}", e);
                        }
                    } else {
                        eprintln!("Compilation failed");
                    }
                } else {
                    eprintln!("Failed to compile.");
                }
            },
            "c"=> {
                let compile_status = if cfg!(target_os = "windows") {
                    Command::new("gcc").args([&info.project_main, "-o", "a.exe"]).status()
                } else {
                    Command::new("gcc").args([&info.project_main, "-o", "a.out"]).status()
                };

                if let Ok(status) = compile_status {
                    if status.success() {
                        let run_status = if cfg!(target_os = "windows") {
                            Command::new("./a.exe").status()
                        } else {
                            Command::new("./a.out").status()
                        };
                        
                        if let Err(e) = run_status {
                            eprintln!("Failed to run compiled program: {}", e);
                        }
                    } else {
                        eprintln!("Compilation failed");
                    }
                } else {
                    eprintln!("Failed to compile.");
                }
            },
            "java" => {
                let compile_status = Command::new("javac").arg(&info.project_main).status();
                if let Ok(status) = compile_status {
                    if status.success() {
                        let class_path = Path::new(&info.project_path).join("src");
                        let class_path_str = class_path.to_str().unwrap();
                        let class_name = "Main";
                        if let Err(e) = Command::new("java").arg("-cp").arg(class_path_str).arg(class_name).status() {
                            eprintln!("Failed to run Java program: {}", e);
                        }
                    } else {
                        eprintln!("Compilation failed");
                    }
                } else {
                    eprintln!("Failed to compile.");
                }
            },
            "javascript" => {
                if let Err(e) = Command::new("node").arg(script_path_str).status() {
                    eprintln!("Failed to execute JavaScript project: {}", e);
                }
            },
            "cs" | "c#" => {
                if let Err(e) = Command::new("dotnet").arg("run").current_dir(&info.project_path).status() {
                    eprintln!("Failed to execute C# project: {}", e);
                }
            },
            "react" => {
                if let Err(e) = Command::new("npm").arg("start").current_dir(&info.project_path).status() {
                    eprintln!("Failed to start React app: {}", e);
                }
            },
            "ruby" => {
                if let Err(e) = Command::new("ruby").arg(script_path_str).status() {
                    eprintln!("Failed to execute Ruby script: {}", e);
                }
            },
            "html" => {
                // Typically, HTML files are opened in a web browser. This example uses the `xdg-open` command on Unix-like systems or `start` on Windows.
                if cfg!(target_os = "windows") {
                    if let Err(e) = Command::new("cmd").args(&["/c", "start", script_path_str]).status() {
                        eprintln!("Failed to open HTML file: {}", e);
                    }
                } else if cfg!(target_os = "macos") {
                    if let Err(e) = Command::new("open").arg(script_path_str).status() {
                        eprintln!("Failed to open HTML file: {}", e);
                    }
                }
                else {
                    if let Err(e) = Command::new("xdg-open").arg(script_path_str).status() {
                        eprintln!("Failed to open HTML file: {}", e);
                    }
                }
            },
            
            
            _ => eprintln!("Unsupported project language."),
        }
    } else {
        println!("Current directory is not a recognized UPM project.");
    }
}


fn add_package(package_name: &str) {
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
        PackageManager::Other(cmd) => {
            let status = Command::new(cmd)
                .arg(package_name)
                .current_dir(current_dir)
                .status()
                .expect("Failed to run package manager command");
            if !status.success() {
                eprintln!("Failed to add package '{}'.", package_name);
            }
        },
    }

    println!("Package '{}' added successfully.", package_name);
}

fn set_main_path(main_path: &str) {
    let current_dir = env::current_dir().unwrap();
    let current_dir_str = clean_path(&current_dir);
    let mut db = load_projects_db();

    let project = db.projects.iter_mut().find(|(_key, value)| {
        current_dir_str.starts_with(&value.project_path)
    });

    match project {
        Some((_, project_info)) => {
            project_info.project_main = main_path.to_string();
            save_projects_db(&db);
            println!("Project main path updated to '{}'", main_path);
        },
        None => println!("No project found in the current directory."),
    }
}
