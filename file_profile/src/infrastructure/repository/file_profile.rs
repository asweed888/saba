use crate::domain::model::file_profile::FileProfile;
use crate::domain::repository::file_profile::FileProfileRepository;
use yaml_rust::{
    Yaml,
    YamlLoader,
    YamlEmitter,
};
use std::fs;
use manifest;

impl<'a> FileProfileRepository<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Self{ file_path }
    }
    pub fn get_file_profile_all(&self) -> Vec<FileProfile> {
        let mut fp: Vec<FileProfile> = vec![];
        let raw = manifest::load(self.file_path);
        if let Some(mf) = raw.get(0) {
            let lang = mf["lang"].as_str().unwrap();
            let arch = mf["arch"].as_str().unwrap();
        }
    }
}
