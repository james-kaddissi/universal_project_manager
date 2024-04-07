use clap::{Arg, Command as ClapCommand };
use std::fs;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::process::Command;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::io;

pub mod project_init;

#[cfg(windows)]
const DB_PATH: &str = "J:\\ultimate_project_manager\\upm_projects.json"; // Adjust the path as necessary

#[cfg(unix)]
const DB_PATH: &str = "/Users/james/WinDesktop/ultimate_project_manager/upm_projects.json"; 


use project_init::create_project;
use project_init::clean_path;
use project_init::add_project_to_db;

#[derive(Deserialize)]
struct Config {
    default_flags: DefaultFlags,
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
    let db_path = Path::new(DB_PATH); // ADJUST PATH TO WHEREVER YOUR ROOT AND JSON IS LOCATED
    let contents = fs::read_to_string(db_path)
        .expect("Failed to read projects database");
    serde_json::from_str(&contents).expect("Failed to deserialize projects database")
}

fn read_config_from(path: &Path) -> Config {
    let config_str = fs::read_to_string(path)
        .expect("Failed to read config file");
    toml::from_str(&config_str).expect("Failed to process config file")
}

fn main() {
    #[cfg(unix)]
    let config_path = Path::new("/Users/james/WinDesktop/ultimate_project_manager/upmconfig.toml"); // ADJUST PATH TO WHEREVER YOUR ROOT AND toml IS LOCATED
    
    #[cfg(windows)]
    let config_path = Path::new("J:\\ultimate_project_manager\\upmconfig.toml"); // ADJUST PATH TO WHEREVER YOUR ROOT AND toml IS LOCATED
    
    let config = read_config_from(config_path);
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
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub_m)) => {
            let project_name = sub_m.get_one::<String>("PROJECT_NAME").unwrap();
            let project_language = sub_m.get_one::<String>("LANGUAGE").unwrap();
        
            let git = sub_m.contains_id("git") || config.default_flags.git;
            let ignore = sub_m.contains_id("ignore") || (config.default_flags.ignore && git);
        
            create_project(project_name, project_language, git, ignore);
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
        }
        _ => {}
    }
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
                } else {
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


fn add_package(package_name: &str) { // only works for python and pip, will come back to this
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let requirements_path = current_dir.join("requirements.txt");


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
        // unix
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


