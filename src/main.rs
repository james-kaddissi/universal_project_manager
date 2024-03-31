use clap::{Arg, Command as ClapCommand };
use std::fs;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::process::Command;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};

pub mod project_init;

use project_init::create_project;

#[derive(Deserialize)]
struct Config {
    DefaultFlags: DefaultFlags,
}

#[derive(Deserialize)]
struct DefaultFlags {
    git: bool,
    ignore: bool,
}

#[derive(Serialize, Deserialize)]
struct ProjectsDb {
    projects: HashMap<String, ProjectInfo>,
}

#[derive(Serialize, Deserialize)]
struct ProjectInfo {
    project_path: String,
    project_language: String,
    project_main: String,
}

fn load_projects_db() -> ProjectsDb {
    let db_path = Path::new("J:\\ultimate_project_manager\\upm_projects.json");
    let contents = fs::read_to_string(db_path)
        .expect("Failed to read projects database");
    serde_json::from_str(&contents).expect("Failed to deserialize projects database")
}

pub fn read_config_from(path: &Path) -> Config {
    let config_str = fs::read_to_string(path)
        .expect("Failed to read config file");
    toml::from_str(&config_str).expect("Failed to process config file")
}

fn main() {
    let matches = ClapCommand::new("upm")
        .version("0.1.1")
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
                    .help("Initializes the project with git"))
                .arg(Arg::new("ignore")
                    .long("ignore")
                    .help("Initializes a .gitignore")
                    .requires("git")) // Makes "ignore" require "git"
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
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub_m)) => {
            let project_name = sub_m.get_one::<String>("PROJECT_NAME").unwrap();
            let project_language = sub_m.get_one::<String>("LANGUAGE").unwrap();
            match sub_m.subcommand() {
                Some(("git", _git_matches)) => {
                    // Initialize project with git
                    match _git_matches.subcommand() {
                        Some(("ignore", _ignore_matches)) => {
                            create_project(project_name, project_language, true, true);
                        },
                        _ => {
                            create_project(project_name, project_language, true, false);
                        },                   
                    }
                },
                _ => {
                    // Create project without git initialization
                    create_project(project_name, project_language, false, false);
                },
            }
        },
        Some(("add", sub_m)) => {
            let package_name = sub_m.get_one::<String>("PACKAGE_NAME").unwrap();
            add_package(package_name);
        },
        Some(("run", sub_m)) => {
            run_project();
        }
        _ => {}
    }
}

fn run_project() {
    let current_dir = env::current_dir().unwrap();
    let current_dir_str = current_dir.to_str().unwrap();
    
    let db = load_projects_db();
    
    let project_info = db.projects.iter().find(|(_key, value)| {
        current_dir_str.starts_with(&value.project_path)
    });

    match project_info {
        Some((_project_name, info)) => {
            let script_path = Path::new(&info.project_path).join(&info.project_main);
            let script_path_str = script_path.to_str().unwrap();

            let command = match info.project_language.as_str() {
                "python" => Command::new("python").arg(script_path_str),
                "rust" => Command::new("cargo").arg("run").current_dir(script_path.parent().unwrap()),
                _ => {
                    eprintln!("Unsupported project language.");
                    return;
                }
            };

            match command.status() {
                Ok(status) if status.success() => {
                    println!("Project ran successfully.");
                },
                Ok(status) => {
                    eprintln!("Project execution failed with exit code: {}", status);
                },
                Err(e) => {
                    eprintln!("Failed to execute project: {}", e);
                },
            }
        },
        None => println!("Current directory is not a recognized UPM project."),
    }
}

fn add_package(package_name: &str) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let requirements_path = current_dir.join("requirements.txt");

    // Append the package name to requirements.txt
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&requirements_path)
        .expect("Failed to open requirements.txt");
    writeln!(file, "{}", package_name).expect("Failed to write to requirements.txt");

    // Determine the correct path for pip based on the operating system
    let pip_path = if cfg!(target_os = "windows") {
        current_dir.join("venv").join("Scripts").join("pip.exe")
    } else {
        // For Unix-like systems
        current_dir.join("venv").join("bin").join("pip")
    };

    // Install the package using pip from the venv
    let status = Command::new(pip_path)
        .args(&["install", package_name])
        .status()
        .expect("Failed to install package");

    if status.success() {
        println!("Package '{}' added successfully.", package_name);
    } else {
        eprintln!("Failed to add package '{}'.", package_name);
    }
}


