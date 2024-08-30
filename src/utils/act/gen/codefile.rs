use crate::domain::model::manifest::MANIFEST;
use std::fs;
use std::path::PathBuf;
use yaml_rust::Yaml;

pub trait Act {
    fn location_action(&self) -> anyhow::Result<()> {
        let manifest = MANIFEST.lock().unwrap();
        let root_path = manifest.root.clone();
        let vec_default: &Vec<Yaml> = &vec![];

        for spec in manifest.spec.clone() {
            let mut workdir = PathBuf::from(&root_path);
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = spec["codefile"].as_vec().unwrap_or(vec_default);
        }

        Ok(())
    }
}