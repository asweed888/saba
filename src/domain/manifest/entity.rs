use yaml_rust::{
    Yaml,
    YamlLoader,
};
use std::fs;
use crate::domain::manifest::root::Root;
use crate::domain::manifest::arch::Arch;
use crate::domain::manifest::lang::Lang;

pub struct Manifest<'a> {
    pub lang: Lang<'a>,
    pub arch: Arch<'a>,
    pub root: Root<'a>,
    pub spec: &'a Vec<Yaml>,
}

pub trait ManifestRepository<'a> {
    fn load(&self) -> Result<Manifest, &str> {
        let f = fs::read_to_string("./saba.yml");
        let s = f.unwrap().to_string();
        let docs = YamlLoader::load_from_str(&s).unwrap();
        let manifest = docs.get(0)
            .ok_or("[ERROR] saba.yml is not found.")?;
        let lang_raw = manifest["lang"].as_str()
            .ok_or("[ERROR] lang is a required field. lang is not set.")?;
        let lang = Lang::new(lang_raw);
        let spec = manifest["spec"].as_vec()
            .ok_or("[ERROR] spec is not set. spec is a required field.")?;
        let arch = Arch::new(manifest["arch"].as_str().unwrap_or("plain"));
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