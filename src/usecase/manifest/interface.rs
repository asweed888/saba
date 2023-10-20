use yaml_rust::Yaml;
use crate::domain::manifest::entity::Manifest;
use std::fs;
use std::fs::File;

pub trait TGenerateFileUseCase<'a> {
    fn location_action(&self, manifest: &'a Manifest) {
        let root_path = manifest.root.path().as_str().clone();
        let mut workdir = root_path.to_string();

        for spec in manifest.spec {
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap();
            let codefile = spec["codefile"].as_vec().unwrap();

            workdir.push_str("/");
            workdir.push_str(location);

            if !upstream.is_empty() {
                self.upstream_action(workdir.clone(), upstream, &manifest);
            }

            if !codefile.is_empty() {

            }
        }
    }
    fn upstream_action(
        &self,
        mut workdir: String,
        upstream: &Vec<Yaml>,
        manifest: &'a Manifest
    ) {
        for u in upstream {
            let dirname = u["name"].as_str().unwrap();
            let upstream = u["upstream"].as_vec().unwrap();
            let codefile = u["codefile"].as_vec().unwrap();
            workdir.push_str("/");
            workdir.push_str(dirname);
            fs::create_dir_all(workdir.clone());

            if !upstream.is_empty() {
                self.upstream_action(workdir.clone(), upstream, manifest);
            }

            if !codefile.is_empty() {
                self.codefile_action(workdir.clone(), codefile, manifest);
            }
        }
    }
    fn codefile_action(
        &self,
        mut workdir: String,
        codefile: &Vec<Yaml>,
        manifest: &'a Manifest
    ) {

    }
}
