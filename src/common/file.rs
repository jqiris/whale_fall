use std::path::{Path, PathBuf};

pub fn path_name(pwd: &Path) -> String {
    let mut name = "".to_string();
    if let Some(file_name) = pwd.file_name() {
        if let Some(nm) = file_name.to_str() {
            name = nm.to_string();
        }
    }
    name
}

pub fn path_str(pwd: &Path) -> String {
    let mut name = "".to_string();
    if let Some(file_name) = pwd.to_str() {
        name = file_name.to_string();
    }
    name
}

pub fn path_join(parts: &[&str]) -> String {
    let mut path = PathBuf::new();
    for part in parts {
        path.push(part);
    }
    path.to_string_lossy().into_owned()
}

pub fn path_parent(path: &str) -> String {
    let path = Path::new(path);
    path.parent().unwrap().to_string_lossy().into_owned()
}

pub fn rel_path(root: &str, path: &str) -> String {
    let mut rel_path = path.strip_prefix(root).unwrap_or(path).to_string();
    rel_path = rel_path.replace("\\", "/");
    rel_path = rel_path.strip_prefix("/").unwrap_or(&rel_path).to_string();
    rel_path
}

pub fn path_exists(path: &str) -> bool {
    let path = Path::new(path);
    path.exists()
}
