use crate::domain::model::manifest::Manifest;
use crate::domain::repository::manifest::ManifestRepository;
use yaml_rust::{
    Yaml,
    YamlLoader,
    YamlEmitter,
};
use std::fs;

impl<'a> ManifestRepository<'a> {
    pub fn new(manifest_path: &'a str) -> Self {
        Self{ manifest_path }
    }
    fn load_raw_data(&self) -> Vec<Yaml> {
        let f = fs::read_to_string(self.manifest_path);
        let s = f.unwrap().to_string();
        let docs = YamlLoader::load_from_str(&s).unwrap();
        docs
    }

}
