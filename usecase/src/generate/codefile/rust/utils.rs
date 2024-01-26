use std::path::PathBuf;

pub fn crate_path(path: &str) -> String {
    let mut p2 = PathBuf::new();
    p2.push(String::from("crate") + path);
    p2.to_str().unwrap().replace("/", "::").to_string()
}

pub fn contains_di_str(path: &str) -> bool {
    path.contains("/di/")
}