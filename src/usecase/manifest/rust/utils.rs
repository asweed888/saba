use crate::usecase::manifest::utils;
use std::path::{Path, PathBuf};

pub fn crate_path(path: &str) -> String {
    let p = Path::new(path);
    let fname = p.file_stem().unwrap();
    let mut p2 = PathBuf::from("/crate");
    p2.push(path.clone());
    p2.push(utils::to_title(fname.to_str().unwrap()));
    p2.to_str().unwrap().to_string()
}
