use std::path::Path;

pub fn clean_path(path: &Path) -> String {
    let mut path_str = path.to_string_lossy().into_owned();
    if cfg!(windows) {
        path_str = path_str.trim_start_matches("\\\\?\\").to_string();
    }
    path_str
}