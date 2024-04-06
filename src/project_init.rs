use std::path::Path;
use std::process::Command;
use serde::{Serialize, Deserialize};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use serde_json;
use std::collections::HashMap;

#[cfg(windows)]
const DB_PATH: &str = "J:\\ultimate_project_manager\\upm_projects.json"; // Adjust the path as necessary

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
    let db_path = Path::new(DB_PATH);
    if !db_path.exists() {
        return ProjectsDb { projects: HashMap::new() };
    }

    let mut file = fs::File::open(db_path).expect("Failed to open projects database");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read projects database");
    serde_json::from_str(&contents).unwrap_or_else(|_| ProjectsDb { projects: HashMap::new() })
}

fn save_projects_db(db: &ProjectsDb) {
    let db_path = Path::new(DB_PATH);
    let contents = serde_json::to_string(db).expect("Failed to serialize projects database");
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(db_path).expect("Failed to open projects database for writing");
    file.write_all(contents.as_bytes()).expect("Failed to write projects database");
}

pub fn add_project_to_db(project_name: &str, project_path: &str, project_language: &str, project_main: &str) {
    let mut db = load_projects_db();
    
    db.projects.insert(project_name.to_string(), ProjectInfo {
        project_path: project_path.to_string(),
        project_language: project_language.to_string(),
        project_main: project_main.to_string(), // Use the provided path
    });
    
    save_projects_db(&db);
}

pub fn clean_path(path: &Path) -> String {
    let mut path_str = path.to_string_lossy().into_owned();
    if cfg!(windows) {
        path_str = path_str.trim_start_matches("\\\\?\\").to_string();
    }
    path_str
}

pub fn create_project(project_name: &str, project_language: &str, git: bool, ignore: bool) {
    match project_language {
        "python" => create_python_project(project_name, git, ignore),
        "cpp" => create_cpp_project(project_name, git, ignore),
        "c++" => create_cpp_project(project_name, git, ignore),
        "c" => create_c_project(project_name, git, ignore),
        "rust" => create_rust_project(project_name, git, ignore),
        "rs" => create_rust_project(project_name, git, ignore),
        "html" => create_html_project(project_name, git, ignore),
        "react" => create_react_project(project_name, git, ignore),
        "java" => create_java_project(project_name, git, ignore),
        "javascript" => create_javascript_project(project_name, git, ignore),
        "ruby" => create_ruby_project(project_name, git, ignore),
        "cs" => create_cs_project(project_name, git, ignore),
        "c#" => create_cs_project(project_name, git, ignore),
        _ => println!("Unsupported project language."),
    }

    let project_main = match project_language {
        "python" => "./src/main.py",
        "cpp" | "c++" => "./src/main.cpp",
        "c" => "./src/main.c",
        "rust" | "rs" => "./src/main.rs",
        "javascript" => "./src/main.js",
        "java" => "./src/Main.java",
        "cs" | "c#" => "./src/Program.cs",
        "react" => "./src/App.js", 
        "ruby" => "./src/main.rb",
        "html" => "./src/index.html",
        _ => "./src/main.txt",
    };

    let project_path = clean_path(&Path::new(project_name).canonicalize().expect("Failed to get absolute path"));
    add_project_to_db(project_name, &project_path, project_language, project_main);
}

fn initialize_git(project_path: &Path, git: bool, ignore: bool) {
    if git {
        Command::new("git")
            .args(&["init", project_path.to_str().unwrap()])
            .status()
            .expect("Failed to initialize git repository");
        println!("Initialized empty Git repository in {}/.git/", project_path.display());

        if ignore {
            let gitignore_path = project_path.join(".gitignore");
            let gitignore_content = "venv/\n__pycache__/\n*.pyc";
            fs::write(gitignore_path, gitignore_content).expect("Failed to create .gitignore");
            println!("Created .gitignore");
        }
    }
}

fn create_cpp_project(project_name: &str, git: bool, ignore: bool) {
    println!("Initializing C++ project...");
    let root_path = Path::new(project_name);
    if root_path.exists() {
        println!("Project {} already exists.", project_name);
        return;
    }

    fs::create_dir_all(root_path.join("src")).expect("Failed to create project directories");
    let mut main_cpp = fs::File::create(root_path.join("src/main.cpp")).expect("Failed to create main.cpp");
    writeln!(main_cpp, "#include <iostream>\n\nint main() {{\n    std::cout << \"Hello, World!\" << std::endl;\n    return 0;\n}}").expect("Failed to write to main.cpp");

    initialize_git(root_path, git, ignore);
    println!("Project {} created successfully.", project_name);
}

fn create_c_project(project_name: &str, git: bool, ignore: bool) {
    println!("Initializing C project...");
    let root_path = Path::new(project_name);
    if root_path.exists() {
        println!("Project {} already exists.", project_name);
        return;
    }

    fs::create_dir_all(root_path.join("src")).expect("Failed to create project directories");
    let mut main_c = fs::File::create(root_path.join("src/main.c")).expect("Failed to create main.c");
    writeln!(main_c, "#include <stdio.h>\n\nint main() {{\n    printf(\"Hello, World!\\n\");\n    return 0;\n}}").expect("Failed to write to main.c");

    initialize_git(root_path, git, ignore);
    println!("Project {} created successfully.", project_name);
}

fn create_rust_project(project_name: &str, git: bool, ignore: bool) {
    println!("Initializing Rust project...");
    Command::new("cargo")
        .args(&["new", project_name, "--bin"])
        .status()
        .expect("Failed to create Rust project with Cargo");

    if git && ignore {
        let gitignore_path = Path::new(project_name).join(".gitignore");
        let gitignore_content = "target/\n**/*.log\n.DS_Store";
        fs::write(gitignore_path, gitignore_content).expect("Failed to create .gitignore");
        println!("Created .gitignore");
    }
    println!("Project {} created successfully.", project_name);
}

fn create_html_project(project_name: &str, git: bool, ignore: bool) {
    println!("Initializing HTML project...");
    let root_path = Path::new(project_name);
    if root_path.exists() {
        println!("Project {} already exists.", project_name);
        return;
    }

    let src_path = root_path.join("src");
    fs::create_dir_all(&src_path).expect("Failed to create project directories");

    let mut index_html = fs::File::create(src_path.join("index.html")).expect("Failed to create index.html");
    writeln!(index_html, "<!DOCTYPE html>\n<html>\n<head>\n    <title>{}</title>\n</head>\n<body>\n    <h1>Hello, World!</h1>\n</body>\n</html>", project_name).expect("Failed to write to index.html");

    initialize_git(root_path, git, ignore);
    println!("Project {} created successfully.", project_name);
}



fn create_react_project(project_name: &str, git: bool, ignore: bool) {
    println!("Initializing React project...");
    Command::new("npx")
        .args(&["create-react-app", project_name])
        .status()
        .expect("Failed to create React project");

    if git && ignore {
        println!("React project initialized with git and .gitignore by default.");
    }
    println!("Project {} created successfully.", project_name);
}

fn create_java_project(project_name: &str, git: bool, ignore: bool) {
    println!("Initializing Java project...");
    let root_path = Path::new(project_name);
    if root_path.exists() {
        println!("Project {} already exists.", project_name);
        return;
    }

    let src_path = root_path.join("src");
    fs::create_dir_all(&src_path).expect("Failed to create project directories");

    let mut main_java = fs::File::create(src_path.join("Main.java")).expect("Failed to create Main.java");
    writeln!(main_java, "public class Main {{\n    public static void main(String[] args) {{\n        System.out.println(\"Hello, World!\");\n    }}\n}}").expect("Failed to write to Main.java");

    initialize_git(root_path, git, ignore);
    println!("Project {} created successfully.", project_name);
}

fn create_javascript_project(project_name: &str, git: bool, ignore: bool) {
    println!("Initializing JavaScript project...");
    let root_path = Path::new(project_name);
    if root_path.exists() {
        println!("Project {} already exists.", project_name);
        return;
    }

    fs::create_dir_all(&root_path).expect("Failed to create project directories");

    let mut index_js = fs::File::create(root_path.join("index.js")).expect("Failed to create index.js");
    writeln!(index_js, "console.log('Hello, World!');").expect("Failed to write to index.js");

    Command::new("npm")
        .args(&["init", "-y"])
        .current_dir(&root_path)
        .status()
        .expect("Failed to initialize npm project");

    initialize_git(root_path, git, ignore);
    println!("Project {} created successfully.", project_name);
}

fn create_ruby_project(project_name: &str, git: bool, ignore: bool) {
    println!("Initializing Ruby project...");
    let root_path = Path::new(project_name);
    if root_path.exists() {
        println!("Project {} already exists.", project_name);
        return;
    }

    fs::create_dir_all(&root_path).expect("Failed to create project directories");

    let mut main_rb = fs::File::create(root_path.join("main.rb")).expect("Failed to create main.rb");
    writeln!(main_rb, "puts 'Hello, World!'").expect("Failed to write to main.rb");

    initialize_git(root_path, git, ignore);
    println!("Project {} created successfully.", project_name);
}

fn create_cs_project(project_name: &str, git: bool, ignore: bool) {
    println!("Initializing C# project...");
    Command::new("dotnet")
        .args(&["new", "console", "-n", project_name])
        .status()
        .expect("Failed to create C# project with dotnet CLI");

    let project_path = Path::new(project_name);
    initialize_git(project_path, git, ignore);
    println!("Project {} created successfully.", project_name);
}

fn create_python_project(project_name: &str, git: bool, ignore: bool) {
    println!("Initializing Python project...");
    let root_path = Path::new(project_name);
    if root_path.exists() {
        println!("Project {} already exists.", project_name);
        return;
    }

    fs::create_dir_all(root_path.join("src")).expect("Failed to create project directories");

    let mut main_py = fs::File::create(root_path.join("src/main.py")).expect("Failed to create main.py");
    writeln!(main_py, "def main():\n    print('Hello, world!')\n\nif __name__ == '__main__':\n    main()").expect("Failed to write to main.py");

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

    println!("CREATING PYTHON VENV");
    create_virtual_env(project_name);

    println!("Project {} created successfully.", project_name);
}
fn create_virtual_env(project_path: &str) {
    Command::new("python3")
        .args(&["-m", "venv", "venv"])
        .current_dir(project_path)
        .status()
        .expect("Failed to create virtual environment");
    println!("Virtual environment created successfully.");
}