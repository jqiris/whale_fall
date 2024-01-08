use std::path::Path;

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
