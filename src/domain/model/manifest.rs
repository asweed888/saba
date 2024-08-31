use yaml_rust::Yaml;
use yaml_rust::YamlLoader;
use anyhow::Context;
use std::default::Default;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Default, Clone)]
pub struct Manifest {
    pub lang: String,
    pub ext: String,
    pub arch: String,
    pub root: String,
    root_raw: String,
    pub main_file: String,
    pub mod_file: String,
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
        let ext = String::new();

        let arch = manifest["arch"]
            .as_str()
            .unwrap_or("plain")
            .to_string();
        let root = String::new();
        let root_raw = manifest["root"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let spec = manifest["spec"]
            .as_vec()
            .context("[ERROR] spec is not set. spec is a required field.")?
            .clone();

        Ok(Self{
            lang,
            ext,
            arch,
            root,
            root_raw,
            main_file: "main".to_string(),
            mod_file: String::new(),
            spec,
        })
    }
    pub fn is_ddd(&self) -> bool {
        self.arch.as_str() == "ddd"
    }
    pub fn set_ext(&mut self, ext: &str) {
        self.ext = ext.to_string();
    }
    pub fn set_root(&mut self, lang_default_root: &str) {
        // saba.ymlにrootの指定が無い場合は言語のデフォルトを設定する
        match self.root_raw.as_str() {
            "" =>  {
                self.root = lang_default_root.to_string();
            },
            _ => {
                self.root = self.root_raw.clone();
            }
        }
    }
    pub fn set_main_file(&mut self, main_file_name: &str) {
        self.main_file = main_file_name.to_string();
    }
    pub fn set_mod_file(&mut self, mod_file_name: &str) {
        self.mod_file = mod_file_name.to_string();
    }
}

pub static MANIFEST: Lazy<Mutex<Manifest>> = Lazy::new(|| {
    let manifest = Manifest::new().expect("[ERROR] Manifest initialization failed.");
    Mutex::new(manifest)
});