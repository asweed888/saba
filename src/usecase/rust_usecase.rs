use crate::domain::model::manifest::MANIFEST;
use crate::utils::act::codefile;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub struct Rust;

impl Rust {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self{})
    }
}


impl codefile::Act for Rust {
    fn gen_location_post(&self) -> anyhow::Result<()> {
        let manifest = MANIFEST.lock().unwrap();
        let root = manifest.root.clone();
        let mut main_rs_content = String::new();

        let main_rs = if PathBuf::from(root.clone() + "lib.rs").exists()
        {
            PathBuf::from(root.clone() + "lib.rs")
        }
        else if PathBuf::from(root.clone() + "main.rs").exists()
        {
            PathBuf::from(root.clone() + "main.rs")
        }
        else {
            let path = PathBuf::from(root.clone() + "main.rs");
            File::create(path.to_str().unwrap())?;
            path
        };


        let main_rs = main_rs.file_name().unwrap().to_str().unwrap_or("");

        main_rs_content.push_str("");
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap_or("");
            let file_stem = path.file_stem().unwrap().to_str().unwrap_or("");

            if file_name == main_rs {
                continue;
            }
        }

        println!("{}", main_rs);
        Ok(())
    }
    fn gen_upstream_post(&self, wd: PathBuf) -> anyhow::Result<()> {
        Ok(())
    }
}
