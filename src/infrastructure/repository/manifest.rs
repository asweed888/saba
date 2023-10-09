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
    pub fn load_manifest(&self) -> Result<Manifest, &str> {
        let f = fs::read_to_string("./saba.yml");
        let s = f.unwrap().to_string();
        let docs = YamlLoader::load_from_str(&s).unwrap();
        let manifest = docs.get(0)
            .ok_or("[ERROR] saba.yml is not found.")?;
        let lang = manifest["lang"].as_str()
            .ok_or("[ERROR] lang is a required field. lang is not set.")?;
        let spec = manifest["spec"].as_vec()
            .ok_or("[ERROR] spec is not set. spec is a required field.")?;
        let arch = manifest["arch"].as_str()
            .unwrap_or("plain");
        let root = manifest["root"].as_str()
            .unwrap_or("");

        Ok(
            Manifest{
                lang,
                arch,
                root,
                spec,
            }
        )
    }
}
