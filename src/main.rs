use clap::{Arg, Command as ClapCommand };
use std::fs;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::process::Command;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::io;

pub mod project_init;

enum PackageManager {
    Pip,
    Cargo,
    Npm,
    Gem,
    Other(String), // For package managers that are just a single command
}

#[cfg(windows)]
const DB_PATH: &str = "J:\\ultimate_project_manager\\upm_projects.json"; // Adjust the path as necessary

#[cfg(unix)]
const DB_PATH: &str = "/Users/james/WinDesktop/ultimate_project_manager/upm_projects.json"; 


use project_init::{ProjectsDb, ProjectInfo, create_project, clean_path, add_project_to_db, save_projects_db};


#[derive(Deserialize, Serialize)]
struct Config {
    default_flags: DefaultFlags,
}

#[derive(Deserialize, Serialize)]
struct DefaultFlags {
    git: bool,
    ignore: bool,
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
        },
        Some(("config", sub_m)) => {
            let modifier = sub_m.get_one::<String>("MODIFIER").unwrap();
            let argument = sub_m.get_one::<String>("ARGUMENT").unwrap();

            if modifier == "main" {
                set_main_path(argument);
            }
            if modifier == "defaults" {
                println!("ahoy");
                set_defaults(argument);
            }
        },
        _ => {}
    }
}

fn set_defaults(argument: &str) {
    println!("Setting defaults for '{}'", argument);

    // Read the current configuration from upmconfig.toml
    let config_path = Path::new("J:\\ultimate_project_manager\\upmconfig.toml"); // Adjust as necessary
    let mut config: Config = match fs::read_to_string(config_path) {
        Ok(contents) => toml::from_str(&contents).expect("Failed to parse config file"),
        Err(e) => {
            eprintln!("Failed to read config file: {}", e);
            return;
        }
    };

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
    let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize to TOML");
    if let Err(e) = fs::write(config_path, toml_str) {
        eprintln!("Failed to write to config file: {}", e);
        return;
    }
    
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
