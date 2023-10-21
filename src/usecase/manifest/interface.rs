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
                self.codefile_action(workdir.clone(), codefile, &manifest);
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
        let ext = manifest.lang.ext().as_str();
        for f in codefile {
            let filename = f["name"].as_str().unwrap();
            workdir.push_str("/");
            workdir.push_str(filename);
            workdir.push_str(".");
            workdir.push_str(ext);

            if self.is_ddd_enabled(manifest) {
                if workdir.contains("domain/model") {
                    self.domain_model_action(workdir.clone(), manifest);
                }
                else if workdir.contains("domain/repository") {
                    self.domain_repository_action(workdir.clone(), manifest);
                }
                else if workdir.contains("infrastructure") {
                    self.infra_action(workdir.clone(), manifest);
                }
                else if workdir.contains("usecase") {
                    self.usecase_action(workdir.clone(), manifest);
                }
                else if workdir.contains("presentation") {
                    self.presentation_action(workdir.clone(), manifest);
                }
            }
            else {
                self.gen_file_default(workdir.clone(), manifest);
            }
        }
    }
    fn is_ddd_enabled(&self, manifest: &'a Manifest) -> bool {
        manifest.arch.is_ddd()
        && manifest.lang.name().as_str() != "bash"
    }
    fn domain_model_action(&self, workdir: String, manifest: &'a Manifest) {
        println!("lang: {}", manifest.lang.name().as_str());
        println!("file path: {}", workdir);
        println!("domain layer model's action");
    }
    fn domain_repository_action(&self, workdir: String, manifest: &'a Manifest) {
        println!("lang: {}", manifest.lang.name().as_str());
        println!("file path: {}", workdir);
        println!("domain layer repository's action");
    }
    fn infra_action(&self, workdir: String, manifest: &'a Manifest) {
        println!("lang: {}", manifest.lang.name().as_str());
        println!("file path: {}", workdir);
        println!("infrastructure layer action");
    }
    fn usecase_action(&self, workdir: String, manifest: &'a Manifest) {
        println!("lang: {}", manifest.lang.name().as_str());
        println!("file path: {}", workdir);
        println!("usecase layer action");
    }
    fn presentation_action(&self, workdir: String, manifest: &'a Manifest) {
        println!("lang: {}", manifest.lang.name().as_str());
        println!("file path: {}", workdir);
        println!("presentation layer action");
    }
    fn gen_file_default(&self, workdir: String, manifest: &'a Manifest) {
        println!("lang: {}", manifest.lang.name().as_str());
        println!("file path: {}", workdir);
        println!("file genarate default action");
    }
}
