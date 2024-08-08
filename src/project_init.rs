use std::path::Path;
use std::process::Command;
use std::fs::{self};
use std::io::{self, Write};
use std::env;

use crate::project_database::{add_project_to_db, load_projects_db};
use crate::util::{clean_path};
use crate::config::{read_config_from};

pub fn create_project(project_name: &str, project_language: &str, git: bool, ignore: bool, venv: bool, license: bool, readme: bool, tests: bool, docs: bool, docker: bool) {
    let lowercase = project_language.to_lowercase();
    match lowercase.as_str() {
        "python" => create_python_project(project_name, git, ignore, venv, license, readme, tests, docs, docker),
        "cpp" => create_cpp_project(project_name, git, ignore, license, readme, tests, docs, docker),
        "c++" => create_cpp_project(project_name, git, ignore, license, readme, tests, docs, docker),
        "c" => create_c_project(project_name, git, ignore, license, readme, tests, docs, docker),
        "rust" => create_rust_project(project_name, git, ignore, license, readme, tests, docs, docker),
        "rs" => create_rust_project(project_name, git, ignore, license, readme, tests, docs, docker),
        "html" => create_html_project(project_name, git, ignore, license, readme, tests, docs, docker),
        "react" => create_react_project(project_name, git, ignore, license, readme, tests, docs, docker),
        "java" => create_java_project(project_name, git, ignore, license, readme, tests, docs, docker),
        "javascript" => create_javascript_project(project_name, git, ignore, license, readme, tests, docs, docker),
        "ruby" => create_ruby_project(project_name, git, ignore, license, readme, tests, docs, docker),
        "cs" => create_cs_project(project_name, git, ignore, license, readme, tests, docs, docker),
        "c#" => create_cs_project(project_name, git, ignore, license, readme, tests, docs, docker),
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

fn initialize_docs(project_path: &Path) {
    let docs_path = project_path.join("docs");
    fs::create_dir_all(&docs_path).expect("Failed to create docs directory");
    let mut index_md = fs::File::create(docs_path.join("index.md")).expect("Failed to create index.md");
    writeln!(index_md, "# Documentation\n\nThis is the documentation for the project.").expect("Failed to write to index.md");
    println!("Initialized docs directory.");
}

fn initialize_tests(project_path: &Path) {
    let tests_path = project_path.join("tests");
    fs::create_dir_all(&tests_path).expect("Failed to create tests directory");
    println!("Initialized tests directory.");
}

fn initialize_license(project_path: &Path) {
    let license_path = project_path.join("LICENSE");
    let license_dir = Path::new("J:\\universal_project_manager\\licenses");
    if !license_dir.exists() {
        eprintln!("No licenses directory found.");
        return;
    }

    let entries = match fs::read_dir(license_dir) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to read licenses directory: {}", err);
            return;
        }
    };
    let config = read_config_from();
    let preferred_license_name = config.preferences.license;
    for entry in entries {
        if let Ok(entry) = entry {
            let license_name = entry.file_name();
            if *license_name == *preferred_license_name {
                let license_content = fs::read_to_string(entry.path())
                    .expect("Failed to read license file");
                fs::write(&license_path, &license_content)
                    .expect("Failed to create LICENSE file");
                println!("Initialized LICENSE file.");
                return;
            }
            println!("{}", license_name.to_string_lossy());
        }
    }

    eprintln!("Preferred license not found in the directory.");
}

fn initialize_readme(project_path: &Path) {
    let readme_path = project_path.join("README.md");
    let readme_content = "# Project Title\n\nDescription of the project.";
    fs::write(readme_path, readme_content).expect("Failed to create README.md");
    println!("Initialized README.md.");
}

fn initialize_docker(project_path: &Path) {
    let docker_path = project_path.join("Dockerfile");
    let docker_content = "";
    fs::write(docker_path, docker_content).expect("Failed to create Dockerfile");
    println!("Initialized Dockerfile.");
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

fn initialize_documents(project_path: &Path, license: bool, readme: bool, tests: bool, docs: bool, docker: bool) {
    if license {
        initialize_license(project_path);
    }
    if readme {
        initialize_readme(project_path);
    }
    if tests {
        initialize_tests(project_path);
    }
    if docs {
        initialize_docs(project_path);
    }
    if docker {
        initialize_docker(project_path);
    }
}


pub fn init_project(project_language: Option<&str>, project_main: Option<&str>) {
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



fn create_cpp_project(project_name: &str, git: bool, ignore: bool, license: bool, readme: bool, tests: bool, docs: bool, docker: bool) {
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
    initialize_documents(root_path, license, readme, tests, docs, docker);
    println!("Project {} created successfully.", project_name);
}

fn create_c_project(project_name: &str, git: bool, ignore: bool, license: bool, readme: bool, tests: bool, docs: bool, docker: bool) {
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
    initialize_documents(root_path, license, readme, tests, docs, docker);
    println!("Project {} created successfully.", project_name);
}

fn create_rust_project(project_name: &str, git: bool, ignore: bool, license: bool, readme: bool, tests: bool, docs: bool, docker: bool) {
    println!("Initializing Rust project...");
    let root_path = Path::new(project_name);
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
    initialize_documents(root_path, license, readme, tests, docs, docker);
    println!("Project {} created successfully.", project_name);
}

fn create_html_project(project_name: &str, git: bool, ignore: bool, license: bool, readme: bool, tests: bool, docs: bool, docker: bool) {
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
    initialize_documents(root_path, license, readme, tests, docs, docker);
    println!("Project {} created successfully.", project_name);
}



fn create_react_project(project_name: &str, git: bool, ignore: bool, license: bool, readme: bool, tests: bool, docs: bool, docker: bool) {
    println!("Initializing React project...");
    let root_path = Path::new(project_name);
    Command::new("npx")
        .args(&["create-react-app", project_name])
        .status()
        .expect("Failed to create React project");

    if git && ignore {
        println!("React project initialized with git and .gitignore by default.");
    }
    initialize_documents(root_path, license, readme, tests, docs, docker);
    println!("Project {} created successfully.", project_name);
}

fn create_java_project(project_name: &str, git: bool, ignore: bool, license: bool, readme: bool, tests: bool, docs: bool, docker: bool) {
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
    initialize_documents(root_path, license, readme, tests, docs, docker);
    println!("Project {} created successfully.", project_name);
}

fn create_javascript_project(project_name: &str, git: bool, ignore: bool, license: bool, readme: bool, tests: bool, docs: bool, docker: bool) {
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
    initialize_documents(root_path, license, readme, tests, docs, docker);
    println!("Project {} created successfully.", project_name);
}

fn create_ruby_project(project_name: &str, git: bool, ignore: bool, license: bool, readme: bool, tests: bool, docs: bool, docker: bool) {
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
    initialize_documents(root_path, license, readme, tests, docs, docker);
    println!("Project {} created successfully.", project_name);
}

fn create_cs_project(project_name: &str, git: bool, ignore: bool, license: bool, readme: bool, tests: bool, docs: bool, docker: bool) {
    println!("Initializing C# project...");
    let root_path = Path::new(project_name);
    Command::new("dotnet")
        .args(&["new", "console", "-n", project_name])
        .status()
        .expect("Failed to create C# project with dotnet CLI");

    let project_path = Path::new(project_name);
    initialize_git(project_path, git, ignore);
    initialize_documents(root_path, license, readme, tests, docs, docker);
    println!("Project {} created successfully.", project_name);
}

fn create_python_project(project_name: &str, git: bool, ignore: bool, venv: bool, license: bool, readme: bool, tests: bool, docs: bool, docker: bool) {
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
    if venv {
        println!("CREATING PYTHON VENV");
        create_virtual_env(project_name);
    }
    initialize_documents(root_path, license, readme, tests, docs, docker);
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