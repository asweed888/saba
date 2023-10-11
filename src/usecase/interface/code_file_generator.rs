use crate::domain::model::manifest::Manifest;
use yaml_rust::Yaml;
use std::fs;
use std::path::Path;

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
    fn get_file_contents(&self) {}
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
