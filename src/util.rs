use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::env;

pub fn clean_path(path: &Path) -> String {
    let mut path_str = path.to_string_lossy().into_owned();
    if cfg!(windows) {
        path_str = path_str.trim_start_matches("\\\\?\\").to_string();
    }
    path_str
}

pub fn get_install_path() -> io::Result<String> {
    let exe_path = env::current_exe()?;

    let exe_path_parent = exe_path.parent().unwrap_or_else(|| Path::new("."));

    let upman_path = exe_path_parent.join("Upman");

    if !upman_path.exists() {
        fs::create_dir_all(&upman_path)?;  
    }

    Ok(upman_path.to_string_lossy().to_string())
}