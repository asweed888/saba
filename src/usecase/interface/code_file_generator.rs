use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::domain::model::manifest::Manifest;
use yaml_rust::Yaml;
use std::fs;
use std::path::Path;

lazy_static! {
    static ref FILE_PATH_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());
}


pub trait CodeFileGenerator<'a> {
    fn location_action(
        &self,
        repository: Manifest<'a>,
        cnf: CodeFileGeneratorConfig<'a>,
    ) {}
    fn upstream_action(
        &self,
        mut workdir: String,
        upstream: &Vec<Yaml>,
        cnf: CodeFileGeneratorConfig<'a>,
    ) {}
    fn codefile_action(
        &self,
        mut workdir: String,
        codefile: &Vec<Yaml>,
        cnf: CodeFileGeneratorConfig<'a>,
    ) {}
    fn is_ddd(&self, repository: Manifest<'a>) -> bool {
        match repository.arch {
            "ddd" => { true }
            _ => { false }
        }
    }
    fn get_root_path(
        &self,
        repository: Manifest<'a>,
        default_root: &str,
    ) -> String {
        match repository.root {
            "" => { String::from(default_root) }
            _ => { String::from(repository.root) }
        }
    }
    fn save_file_path(&self, path: &str) {
        let mut data = FILE_PATH_LIST.lock().unwrap();
        data.push(String::from(path));
    }
    fn get_file_path_list(&self) -> Vec<String> {
        let mut data = FILE_PATH_LIST.lock().unwrap();
        *data
    }
}

pub struct CodeFileGeneratorConfig<'a> {
    pub ext: &'a str,
    pub default_root: &'a str,
}

impl<'a> CodeFileGeneratorConfig<'a> {
    pub fn new(ext: &'a str, default_root: &'a str) -> Self {
        Self{
            ext,
            default_root,
        }
    }
}
