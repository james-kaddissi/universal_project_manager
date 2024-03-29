use clap::{Arg, Command as ClapCommand };
use std::fs;
use std::env;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    DefaultFlags: DefaultFlags,
}

#[derive(Deserialize)]
struct DefaultFlags {
    git: bool,
    ignore: bool,
}

fn read_config() -> Config {
    let config_str = fs::read_to_string("upmconfig.toml")
        .expect("Failed to read upmconfig.toml");
    toml::from_str(&config_str).expect("Failed to process upmconfig.toml")
}

fn main() {
    let matches = ClapCommand::new("upm")
        .version("0.1.0")
        .about("Manages programming projects")
        .subcommand(
            ClapCommand::new("new")
                .about("Creates a new Python project")
                .arg(Arg::new("PROJECT_NAME").help("The name of the project").required(true).index(1))
                .subcommand(
                    ClapCommand::new("git")
                        .about("Initializes the project with git")
                        .subcommand(
                            ClapCommand::new("ignore")
                                .about("Initializes a .gitignore")
                        )
                )
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
                .about("Runs the file located at ./src/main.py")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub_m)) => {
            let project_name = sub_m.get_one::<String>("PROJECT_NAME").unwrap();
            match sub_m.subcommand() {
                Some(("git", _git_matches)) => {
                    // Initialize project with git
                    match _git_matches.subcommand() {
                        Some(("ignore", _ignore_matches)) => {
                            create_project(project_name, true, true);
                        },
                        _ => {
                            create_project(project_name, true, false);
                        },
                    }                    
                },
                _ => {
                    // Create project without git initialization
                    create_project(project_name, false, false);
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
    let script_path = "./src/main.py";

    match Command::new("python")
        .arg(script_path)
        .status() {
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
}

fn create_project(project_name: &str, git: bool, ignore: bool) {
    let root_path = Path::new(project_name);
    if root_path.exists() {
        println!("Project {} already exists.", project_name);
        return;
    }

    // Create project root directory
    fs::create_dir_all(root_path.join("src")).expect("Failed to create project directories");

    // Create main.py inside src
    let mut main_py = fs::File::create(root_path.join("src/main.py")).expect("Failed to create main.py");
    writeln!(main_py, "def main():\n    print('Hello, world!')\n\nif __name__ == '__main__':\n    main()").expect("Failed to write to main.py");

    // Create requirements.txt
    let _ = fs::File::create(root_path.join("requirements.txt")).expect("Failed to create requirements.txt");

    if git {
        Command::new("git")
            .args(&["init", project_name])
            .status()
            .expect("Failed to initialize git repository");
        println!("Initialized empty Git repository in {}/.git/", project_name);
    }

    if ignore {
        let gitignore_path = Path::new(project_name).join(".gitignore");
        let gitignore_content = if ignore { "venv/\n__pycache__/\n*.pyc" } else { "" }; // Customize as needed
        fs::write(gitignore_path, gitignore_content).expect("Failed to create .gitignore");
        println!("Created .gitignore");
    }

    // Create virtual environment
    println!("CREATING PYTHON VENV");
    create_virtual_env(project_name);

    println!("Project {} created successfully.", project_name);
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

fn create_virtual_env(project_path: &str) {
    Command::new("python3")
        .args(&["-m", "venv", "venv"])
        .current_dir(project_path)
        .status()
        .expect("Failed to create virtual environment");
    println!("Virtual environment created successfully.");
}
