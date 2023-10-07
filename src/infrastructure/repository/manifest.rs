use crate::domain::model::manifest::Manifest;
use crate::domain::repository::manifest::ManifestRepository;
use yaml_rust::{
    Yaml,
    YamlLoader,
};
use std::fs;


impl ManifestRepository {
    pub fn new() -> Self {
        Self{}
    }
    pub fn load_manifest(&self) -> Vec<Yaml> {
        let f = fs::read_to_string("./saba.yml");
        let s = f.unwrap().to_string();
        let docs = YamlLoader::load_from_str(&s).unwrap();
        docs
    }
}
