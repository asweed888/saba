use crate::domain::manifest::entity::{
    Manifest,
    ManifestRepository,
};
use yaml_rust::YamlLoader;
use anyhow::{Context, Result};
use std::fs;
use crate::domain::manifest::root::Root;
use crate::domain::manifest::arch::Arch;
use crate::domain::manifest::lang::Lang;

impl ManifestRepository {
    pub fn new() -> Self {
        Self{}
    }
    pub fn load(&self) -> Result<Manifest> {
        let f = fs::read_to_string("./saba.yml");
        let s = f.unwrap().to_string();
        let docs = YamlLoader::load_from_str(&s).unwrap();
        let manifest_raw = docs.get(0).clone()
            .context("[ERROR] saba.yml is not found.")?;
        let manifest = manifest_raw.clone();
        let lang_raw = manifest["lang"].as_str()
            .context("[ERROR] lang is a required field. lang is not set.")?;
        let lang = Lang::new(String::from(lang_raw));
        let spec_raw = manifest["spec"].as_vec()
            .context("[ERROR] spec is not set. spec is a required field.")?;
        let spec = spec_raw.clone();
        let arch = Arch::new(String::from(
            manifest["arch"].as_str().unwrap_or("plain"),
        ));
        let root = Root::new(String::from(
            manifest["root"].as_str().unwrap_or(""),
        ));

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
