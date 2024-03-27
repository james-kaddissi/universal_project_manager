use clap::{Arg, Command as ClapCommand};
use std::fs;
use std::env;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let matches = ClapCommand::new("Ultimate Project Manager")
        .version("0.1.0")
        .author("James Kaddissi")
        .about("Manages Python projects")
        .subcommand(
            ClapCommand::new("new")
                .about("Creates a new Python project")
                .arg(Arg::new("PROJECT_NAME")
                    .help("The name of the project")
                    .required(true)
                    .index(1)),
        )
        .subcommand(
            ClapCommand::new("add")
                .about("Adds a package to the project")
                .arg(Arg::new("PACKAGE_NAME")
                    .help("The name of the package to add")
                    .required(true)
                    .index(1)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("new", new_matches)) => {
            let project_name = new_matches.get_one::<String>("PROJECT_NAME").unwrap();
            create_project(project_name);
        },
        Some(("add", add_matches)) => {
            let package_name = add_matches.get_one::<String>("PACKAGE_NAME").unwrap();
            add_package(package_name);
        },
        _ => {}
    }
}
fn create_project(project_name: &str) {
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

    // Create virtual environment
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
