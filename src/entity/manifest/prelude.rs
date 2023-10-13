use yaml_rust::{
    Yaml,
    YamlLoader,
};
use std::fs;
use crate::entity::manifest::root::Root;

pub struct Manifest<'a> {
    pub lang: &'a str,
    pub arch: &'a str,
    pub root: Root<'a>,
    pub spec: &'a Vec<Yaml>,
}

impl<'a> Manifest<'a> {
    pub fn new(
        lang: &'a str,
        arch: &'a str,
        root: Root<'a>,
        spec: &'a Vec<Yaml>,
    ) -> Self {
        Self{
            lang,
            arch,
            root,
            spec,
        }
    }
}

pub trait ManifestRepository<'a> {
    fn load(&self) -> Result<Manifest, &str> {
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
        let root = Root::new(manifest["root"].as_str().unwrap_or(""));

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

pub trait ManifestUseCase<'a> {
    fn location_action(&self) {

    }
}

impl<'a> ManifestRepository<'a> for Manifest<'a> {}
impl<'a> ManifestUseCase<'a> for Manifest<'a> {}