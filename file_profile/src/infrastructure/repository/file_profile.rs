use crate::domain::model::file_profile::FileProfile;
use crate::domain::repository::file_profile::FileProfileRepository;
use yaml_rust::{
    Yaml,
    YamlLoader,
    YamlEmitter,
};
use std::fs;

impl<'a> FileProfileRepository<'a> {
    pub fn new(manifest: &'a str) -> Self {
        Self{ manifest }
    }
    pub fn get_file_profile_all(&self) -> Vec<FileProfile> {
        let fp: Vec<FileProfile> = vec![];
        let raw = self.load_manifest();
        if let Some(manifest) = raw.get(0);

    }
    fn lang(&self, lang: String) -> String {

    }
    fn load_manifest(&self) -> Vec<Yaml> {
        let f = fs::read_to_string(self.manifest);
        let s = f.unwrap().to_string();
        let docs = YamlLoader::load_from_str(&s).unwrap();
        docs
    }
}
