use clap::{Arg, Command as ClapCommand };
pub mod project_init;
pub mod project_database;
pub mod project_management;
pub mod secrets;
pub mod util;
pub mod config;
pub mod scripts;
pub mod packages;
pub mod templates;
pub mod list;

use crate::project_init::{create_project, init_project};
use crate::util::{clean_path};
use crate::project_database::{load_projects_db};
use crate::config::{read_config_from, set_license, set_defaults, set_editor};
use crate::secrets::{secrets_manager};
use crate::scripts::{add_script, delete_script, save_script};
use crate::packages::{add_package};
use crate::project_management::{open_project, delete_project, run_project, set_main_path};
use crate::templates::{template_manager};
use crate::list::{list_manager};

fn main() {
    let config = read_config_from();
    let matches = ClapCommand::new("upman")
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
                    .action(clap::ArgAction::SetFalse))
                .arg(Arg::new("ignore")
                    .long("ignore")
                    .help("Initializes a .gitignore")
                    .requires("git")
                    .action(clap::ArgAction::SetFalse)) 
                .arg(Arg::new("venv")
                    .long("venv")
                    .help("Initializes a venv")
                    .action(clap::ArgAction::SetFalse))
                .arg(Arg::new("license")
                    .long("license")
                    .help("Initializes a license. Uses default license if no argument is provided.")
                    .action(clap::ArgAction::SetFalse))
                .arg(Arg::new("readme")
                    .long("readme")
                    .help("Initializes a readme. Uses default readme.")
                    .action(clap::ArgAction::SetFalse))
                .arg(Arg::new("tests")
                    .long("tests")
                    .help("Initializes a tests directory.")
                    .action(clap::ArgAction::SetFalse))
                .arg(Arg::new("docs")
                    .long("docs")
                    .help("Initializes a docs directory.")
                    .action(clap::ArgAction::SetFalse))
                .arg(Arg::new("docker")
                    .long("docker")
                    .help("Initializes the project with docker")
                    .action(clap::ArgAction::SetFalse))
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
            ClapCommand::new("script")
                .about("Manages specific scripts.")
                .arg(Arg::new("ACTION")
                    .help("The action to perform on the script")
                    .required(true)
                    .index(1))
                .arg(Arg::new("SCRIPT_NAME")
                    .help("The name of the script")
                    .required(true)
                    .index(2))
                .arg(Arg::new("SCRIPT_PATH")
                    .help("Specifies the path of the script")
                    .required(false)
                    .index(3))
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
        .subcommand(
            ClapCommand::new("open")
                .about("Opens the project in the default editor")
                .arg(Arg::new("PROJECT")
                    .help("The name of the project to open")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            ClapCommand::new("delete")
                .about("Deletes the project specified.")
                .arg(Arg::new("PROJECT")
                    .help("The name of the project to delete")
                    .required(true)
                    .index(1))
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
            if config.warnings.creation{
                println!("Project creation requires the necessary dependencies to be installed. Errors may occur if you do not have the language installed.");
                println!("To disable this warning run 'upman config warnings creation' to toggle the warning, or manually set it to false in the upmconfig.toml file.");
            }
            create_project(project_name, project_language, git, ignore, venv, license, readme, tests, docs, docker);
        },
        Some(("add", sub_m)) => {
            let package_name = sub_m.get_one::<String>("PACKAGE_NAME").unwrap();
            if config.warnings.add {
                println!("Adding a package requires the necessary package manager for that language to be installed. Some languages may not have a package manager.");
                println!("To disable this warning run 'upman config warnings add' to toggle the warning, or manually set it to false in the upmconfig.toml file.");
            }
            add_package(package_name);
        },
        Some(("run", _)) => {
            if config.warnings.run {
                println!("Running a project requires the necessary dependencies to be installed. Errors may occur if you do not have the language or other necessary compilers/interpreters installed.");
                println!("To disable this warning run 'upman config warnings run' to toggle the warning, or manually set it to false in the upmconfig.toml file.");
            }
            run_project();
        },
        Some(("script", sub_m)) => {
            let action = sub_m.get_one::<String>("ACTION").unwrap();
            let script_name = sub_m.get_one::<String>("SCRIPT_NAME").unwrap();
            let script_path = sub_m.get_one::<String>("SCRIPT_PATH");

            if action == "save" {
                save_script(script_name, script_path.map(String::as_str));
            }
            if action == "delete" {
                delete_script(script_name);
            }
            if action == "add" {
                add_script(script_name);
            }
        },
        Some(("init", sub_m)) => {
            let project_language = sub_m.get_one::<String>("LANGUAGE");
            let project_main = sub_m.get_one::<String>("MAIN");
            if config.warnings.run {
                println!("Initializing a project requires the necessary dependencies to be installed. Errors may occur if you do not have the language installed.");
                println!("To disable this warning run 'upman config warnings init' to toggle the warning, or manually set it to false in the upmconfig.toml file.");
            }
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
        Some(("open", sub_m)) => {
            let project = sub_m.get_one::<String>("PROJECT").unwrap();
            open_project(project);
        },
        Some(("delete", sub_m)) => {
            let project = sub_m.get_one::<String>("PROJECT").unwrap();
            delete_project(project);
        },
        _ => {}
    }
}