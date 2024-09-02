use std::path::PathBuf;
use std::fs::File;

pub fn crate_path(path: &str) -> String {
    let mut p2 = PathBuf::new();
    p2.push(String::from("crate") + path);
    p2.to_str().unwrap().replace("/", "::").to_string()
}

pub fn contains_di_str(path: &str) -> bool {
    path.contains("/di/")
}

pub fn contains_traits_str(path: &str) -> bool {
    path.contains("/traits/")
}

pub fn contains_act_str(path: &str) -> bool {
    path.contains("/act/")
}

pub fn main_rs(root: String) -> anyhow::Result<String> {
    let main_rs = if PathBuf::from(root.clone() + "/" + "lib.rs").exists()
    {
        PathBuf::from(root.clone() + "/" + "lib.rs")
    }
    else if PathBuf::from(root.clone() + "/" + "main.rs").exists()
    {
        PathBuf::from(root.clone() + "/" + "main.rs")
    }
    else {
        let path = PathBuf::from(root.clone() + "/" + "main.rs");
        File::create(path.to_str().unwrap())?;
        path
    };


    let main_rs = main_rs
        .file_name()
        .unwrap()
        .to_str()
        .expect("[ERROR] Failed to obtain main.rs or lib.rs information.");


    Ok(main_rs.to_string())
}

pub fn mod_rs(wd: PathBuf) -> anyhow::Result<String> {
    let workdir = wd.to_str().unwrap().to_string();
    let mod_rs = if PathBuf::from(workdir.clone() + "/" + "mod.rs").exists() {
        PathBuf::from(workdir.clone() + "/" + "mod.rs")
    }
    else {
        let path = PathBuf::from(workdir.clone() + "/" + "mod.rs");
        File::create(path.to_str().unwrap())?;
        path
    };
    let mod_rs = mod_rs
        .file_name()
        .unwrap()
        .to_str()
        .expect("[ERROR] Failed to obtain mod.rs information.");

    Ok(mod_rs.to_string())
}
