use yaml_rust::Yaml;
use yaml_rust::YamlLoader;
use anyhow::Context;
use std::default::Default;
use once_cell::sync::Lazy;

#[derive(Default)]
pub struct Manifest {
    pub lang: String,
    pub arch: String,
    pub root: String,
    pub spec: Vec<Yaml>,
}

impl Manifest {
    pub fn new() -> anyhow::Result<Self> {
        let file = std::fs::read_to_string("./saba.yml");
        let s = file.unwrap().to_string();
        let file_content = YamlLoader::load_from_str(&s).unwrap();
        let manifest = file_content.get(0).clone()
            .context("[ERROR] saba.yml is not found.")?;

        let lang = manifest["lang"]
            .as_str()
            .context("[ERROR] lang is a required field. lang is not set.")?
            .to_string();
        let arch = manifest["arch"]
            .as_str()
            .unwrap_or("plain")
            .to_string();
        let root = manifest["root"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let spec = manifest["spec"]
            .as_vec()
            .context("[ERROR] spec is not set. spec is a required field.")?
            .clone();

        Ok(Self{
            lang,
            arch,
            root,
            spec,
        })
    }
}

pub static MANIFEST: Lazy<anyhow::Result<Manifest>> = Lazy::new(|| Manifest::new());