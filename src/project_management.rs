use std::process::{Command, Stdio};
use std::path::Path;
use crate::project_database::{load_projects_db, save_projects_db};
use crate::config::read_config_from;
use crate::util::{clean_path, get_install_path};
use std::env;
use std::fs::{self, Permissions};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt; 

pub fn delete_project(project: &str) {
    let mut db = load_projects_db();
    if let Some(project_info) = db.projects.remove(project) {
        save_projects_db(&db);
        println!("Project '{}' deleted from database successfully.", project);

        let project_path = Path::new(&project_info.project_path);
        
        if project_path.exists() {
            if let Err(err) = fs::remove_dir_all(project_path) {
                println!("Failed to delete project '{}': {}", project, err);
            } else {
                println!("Project '{}' deleted from system successfully.", project);
            }
        } else {
            println!("Project '{}' path not found.", project);
        }
    } else {
        println!("Project '{}' not found in database.", project);
    }
}


pub fn open_project(project: &str) {
    let db = load_projects_db();
    let config = read_config_from();

    if let Some(project_info) = db.projects.get(project) {
        let project_path = &project_info.project_path;

        let install_path = get_install_path().unwrap(); 
        let script_path_str = if cfg!(windows) {
            format!("{}/open_project.bat", install_path)
        } else {
            format!("{}/open_project.sh", install_path)
        };

        let script_path = Path::new(&script_path_str);

        if !script_path.exists() {
            let script_content = if cfg!(windows) {
                r"@echo off
        start cmd /k cd /d %~1"
            } else {
                r"#!/bin/bash
        cd $1"
            };
        
            if let Err(err) = fs::write(&script_path, script_content) {
                eprintln!("Failed to create script {:?}: {}", script_path, err);
                return;
            }
        
            #[cfg(unix)]
            {
                if let Err(err) = fs::set_permissions(script_path, Permissions::from_mode(0o755)) {
                    eprintln!("Failed to set permissions for script '{}': {}", script_path.display(), err);
                }
            }
        
            println!("Created script: {:?}", script_path);
        }

        let status = if cfg!(windows) {
            Command::new("cmd")
                .arg("/C")
                .arg(&script_path)
                .arg(project_path)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(format!("{} {}", script_path.display(), project_path))
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
        };

        match status {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Failed to execute script {:?}", script_path);
                }
            }
            Err(err) => {
                eprintln!("Failed to run command: {}", err);
            }
        }
    } else {
        println!("Project '{}' not found. Ensure it is recognized as a UPM project by using 'upm init' in the root of the project.", project);
    }
}

pub fn run_project() {
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
            "swift" => {
                if let Err(e) = Command::new("swift").args(&["run"]).status() {
                    eprintln!("Failed to execute Swift project: {}", e);
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
            "js" | "javascript" => {
                if let Err(e) = Command::new("node").arg(script_path_str).status() {
                    eprintln!("Failed to execute JavaScript project: {}", e);
                }
            },
            "ts" | "typescript" => {
                if let Err(e) = Command::new("C:\\Program Files\\nodejs\\npx.cmd").args(&["tsc"]).status() {
                    eprintln!("Failed to compile TypeScript: {}", e);
                    return;
                }
                if let Err(e) = Command::new("node").arg(script_path_str).status() {
                    eprintln!("Failed to execute TypeScript project: {}", e);
                }
            }
            "go" => {
                if let Err(e) = Command::new("go").args(&["run", script_path_str]).status() {
                    eprintln!("Failed to execute Go project: {}", e);
                }
            }
            "cs" | "c#" => {
                if let Err(e) = Command::new("dotnet").arg("run").current_dir(&info.project_path).status() {
                    eprintln!("Failed to execute C# project: {}", e);
                }
            },
            "react" => {
                if let Err(e) = Command::new("C:\\Program Files\\nodejs\\npm.cmd").arg("start").current_dir(&info.project_path).status() {
                    eprintln!("Failed to start React app: {}", e);
                }
            },
            "ruby" => {
                if let Err(e) = Command::new("ruby").arg(script_path_str).status() {
                    eprintln!("Failed to execute Ruby script: {}", e);
                }
            },
            "dart" => {
                if let Err(e) = Command::new("dart").arg("run").arg(script_path_str).status() {
                    eprintln!("Failed to execute Ruby script: {}", e);
                }
            },
            "shell" => {
                #[cfg(unix)]
                if let Err(e) = Command::new("sh").arg(script_path_str).status() {
                    eprintln!("Failed to execute shell script: {}", e);
                }

                #[cfg(windows)]
                println!("Shell scripts are not supported on Windows.");
            },
            "r" => {
                println!("Open the R script in RStudio or R interpreter.");
            },

            "scala" => {
                println!("Open the Scala script in SBT.");
            },
            "perl" => {
                if let Err(e) = Command::new("perl").arg(script_path_str).status() {
                    eprintln!("Failed to execute Perl script: {}", e);
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

pub fn set_main_path(main_path: &str) {
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