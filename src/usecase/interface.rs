use crate::domain::model::manifest::Manifest;
use yaml_rust::Yaml;

pub trait CodeFileGenerator<'a> {
    fn location_action(&self, repository: Manifest<'a>) {}
    fn upstream_action(&self, mut workdir: String, upstream: &Vec<Yaml>) {}
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
