use clap::{Arg, Command as ClapCommand};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let matches = ClapCommand::new("Ultimate Project Manager")
        .version("0.1.0")
        .author("Your Name")
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
                .about("Adds a package to the project"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("new", new_matches)) => {
            let project_name = new_matches.get_one::<String>("PROJECT_NAME").unwrap();
            create_project(project_name);
        },
        Some(("add", _)) => {
            add_package();
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

fn add_package() {
    println!("Adding package");
    // Implement package addition logic here
}

fn create_virtual_env(project_path: &str) {
    Command::new("python3")
        .args(&["-m", "venv", "venv"])
        .current_dir(project_path)
        .status()
        .expect("Failed to create virtual environment");
    println!("Virtual environment created successfully.");
}
