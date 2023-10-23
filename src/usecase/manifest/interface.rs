use yaml_rust::Yaml;
use crate::domain::manifest::entity::Manifest;
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Clone, Copy,Debug)]
pub struct WorkDir<'a> {
    pub path: &'a str,
    root: &'a str,
}

impl<'a> WorkDir<'a> {
    pub fn new() -> Self {
        let path = "";
        let root = "";
        Self{
            path,
            root,
        }
    }
    pub fn init(&mut self, root: &'a str) {
        self.root = root.clone();
        self.path = root.clone();
    }
    pub fn reset(&mut self) {
        self.path = self.root.clone();
    }
    pub fn path_push_str(&mut self, path: &str) {
        self.path.to_string().push_str(path);
    }
    pub fn fname(&mut self) -> Option<String> {
        let path = Path::new(self.path);
        Some(path.file_name().unwrap().to_str().unwrap_or("").to_string())
    }
    pub fn pkgname(&mut self) -> Option<String> {
        let path = Path::new(self.path);
        let root_path = self.root;
        let parent = path.parent()
            .unwrap()
            .to_str()
            .unwrap_or("");

        match root_path {
            "." => {
                if parent != "." {
                    return Some(parent.to_string())
                }
                else {
                    return None
                }
            }
            _ => {
                let root = root_path.replace("./", "");
                if parent != root.as_str() {
                    return Some(parent.to_string())
                }
                else {
                    return None
                }
            }
        }

    }
}

pub trait TGenerateFileUseCase<'a> {
    fn location_action(&self, manifest: &'a Manifest) {
        let root_path = manifest.root.get_path();
        let mut workdir = WorkDir::new();
        workdir.init(root_path.as_str());


        for spec in manifest.spec {
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap();
            let codefile = spec["codefile"].as_vec().unwrap();

            workdir.path_push_str("/");
            workdir.path_push_str(location);

            if !upstream.is_empty() {
                self.upstream_action(workdir, upstream, &manifest);
            }

            if !codefile.is_empty() {
                self.codefile_action(workdir, codefile, &manifest);
            }
        }
    }
    fn upstream_action(
        &self,
        mut workdir: WorkDir,
        upstream: &Vec<Yaml>,
        manifest: &'a Manifest
    ) {
        for u in upstream {
            let dirname = u["name"].as_str().unwrap();
            let upstream = u["upstream"].as_vec().unwrap();
            let codefile = u["codefile"].as_vec().unwrap();
            workdir.path_push_str("/");
            workdir.path_push_str(dirname);
            fs::create_dir_all(workdir.path);

            if !upstream.is_empty() {
                self.upstream_action(workdir, upstream, manifest);
            }

            if !codefile.is_empty() {
                self.codefile_action(workdir, codefile, manifest);
            }
        }
    }
    fn codefile_action(
        &self,
        mut workdir: WorkDir,
        codefile: &Vec<Yaml>,
        manifest: &'a Manifest
    ) {
        let ext = manifest.lang.ext().as_str();
        for f in codefile {
            let filename = f["name"].as_str().unwrap();
            workdir.path_push_str("/");
            workdir.path_push_str(filename);
            workdir.path_push_str(".");
            workdir.path_push_str(ext);

            if self.is_ddd_enabled(manifest) {
                if workdir.path.contains("domain/model") {
                    self.domain_model_action(workdir);
                }
                else if workdir.path.contains("domain/repository") {
                    self.domain_repository_action(workdir);
                }
                else if workdir.path.contains("infrastructure") {
                    self.infra_action(workdir);
                }
                else if workdir.path.contains("usecase") {
                    self.usecase_action(workdir);
                }
                else if workdir.path.contains("presentation") {
                    self.presentation_action(workdir);
                }
            }
            else {
                self.gen_file_default(workdir);
            }
        }
    }
    fn is_ddd_enabled(&self, manifest: &'a Manifest) -> bool {
        manifest.arch.is_ddd()
        && manifest.lang.name().as_str() != "bash"
    }
    fn domain_model_action(
        &self,
        workdir: WorkDir,
    ) -> Result<()> {
        println!("generating file: {}", workdir.path);
        Ok(())
    }
    fn domain_repository_action(
        &self,
        workdir: WorkDir,
    ) -> Result<()> {
        println!("generating file: {}", workdir.path);
        Ok(())
    }
    fn infra_action(&self,
        workdir: WorkDir,
    ) -> Result<()> {
        println!("generating file: {}", workdir.path);
        Ok(())
    }
    fn usecase_action(&self,
        workdir: WorkDir,
    ) -> Result<()> {
        println!("generating file: {}", workdir.path);
        Ok(())
    }
    fn presentation_action(&self,
        workdir: WorkDir,
    ) -> Result<()> {
        println!("generating file: {}", workdir.path);
        Ok(())
    }
    fn gen_file_default(&self,
        workdir: WorkDir,
    ) -> Result<()> {
        println!("generating file: {}", workdir.path);
        Ok(())
    }
}
