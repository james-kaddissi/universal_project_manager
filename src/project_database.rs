use std::path::{Path};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::util::get_install_path;

#[derive(Serialize, Deserialize)]
pub struct ProjectsDb {
    pub projects: HashMap<String, ProjectInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectInfo {
    pub project_path: String,
    pub project_language: String,
    pub project_main: String,
}

fn db_path() -> String {
    let exe_dir = get_install_path().expect("Failed to get install path"); 
    format!("{}/upm_projects.json", exe_dir)
}

pub fn load_projects_db() -> ProjectsDb {
    let path_str = db_path();
    let db_path = Path::new(&path_str);
    if !db_path.exists() {
        return ProjectsDb { projects: HashMap::new() };
    }

    let mut file = fs::File::open(db_path).expect("Failed to open projects database");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read projects database");
    serde_json::from_str(&contents).unwrap_or_else(|_| ProjectsDb { projects: HashMap::new() })
}
pub fn save_projects_db(db: &ProjectsDb) {
    let path_str = db_path();
    let db_path = Path::new(&path_str);
    let contents = serde_json::to_string(db).expect("Failed to serialize projects database");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true) 
        .truncate(true)
        .open(db_path)
        .expect("Failed to open projects database for writing");

    file.write_all(contents.as_bytes()).expect("Failed to write projects database");
}

pub fn add_project_to_db(project_name: &str, project_path: &str, project_language: &str, project_main: &str) {
    let mut db = load_projects_db();

    db.projects.insert(project_name.to_string(), ProjectInfo {
        project_path: project_path.to_string(),
        project_language: project_language.to_string(),
        project_main: project_main.to_string(),
    });

    save_projects_db(&db);
}