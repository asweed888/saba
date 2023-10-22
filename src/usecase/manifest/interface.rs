use yaml_rust::Yaml;
use crate::domain::manifest::entity::Manifest;
use std::fs;
use std::path::Path;

pub struct WorkDir {
    pub path: String,
    root: String,
}

impl WorkDir {
    pub fn new(
        path_orig: &str,
    ) -> Self {
        let path = String::from(path_orig);
        let root = String::from(path_orig);
        Self{
            path,
            root,
        }
    }
    pub fn path_push_str(&mut self, path: &str) {
        self.path.push_str(path);
    }
    pub fn fname(&self) -> String {
        let path = Path::new(self.path.as_str());
        path.file_name().unwrap().to_str().unwrap().to_string()
    }
    pub fn pkgname(&self) -> Option<String> {
        let path = Path::new(self.path.as_str());
        let root_path = self.root.as_str();
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
        let mut workdir = WorkDir::new(&root_path.as_str());

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
            workdir.path.push_str("/");
            workdir.path.push_str(dirname);
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
            workdir.path.push_str("/");
            workdir.path.push_str(filename);
            workdir.path.push_str(".");
            workdir.path.push_str(ext);

            if self.is_ddd_enabled(manifest) {
                if workdir.path.contains("domain/model") {
                    self.domain_model_action(workdir, manifest);
                }
                else if workdir.path.contains("domain/repository") {
                    self.domain_repository_action(workdir, manifest);
                }
                else if workdir.path.contains("infrastructure") {
                    self.infra_action(workdir, manifest);
                }
                else if workdir.path.contains("usecase") {
                    self.usecase_action(workdir, manifest);
                }
                else if workdir.path.contains("presentation") {
                    self.presentation_action(workdir, manifest);
                }
            }
            else {
                self.gen_file_default(workdir, manifest);
            }
        }
    }
    fn is_ddd_enabled(&self, manifest: &'a Manifest) -> bool {
        manifest.arch.is_ddd()
        && manifest.lang.name().as_str() != "bash"
    }
    fn domain_model_action(&self, workdir: WorkDir, manifest: &'a Manifest) {
        println!("lang: {}", manifest.lang.name().as_str());
        println!("file path: {}", workdir.path);
        println!("domain layer model's action");
    }
    fn domain_repository_action(&self, workdir: WorkDir, manifest: &'a Manifest) {
        println!("lang: {}", manifest.lang.name().as_str());
        println!("file path: {}", workdir.path);
        println!("domain layer repository's action");
    }
    fn infra_action(&self, workdir: WorkDir, manifest: &'a Manifest) {
        println!("lang: {}", manifest.lang.name().as_str());
        println!("file path: {}", workdir.path);
        println!("infrastructure layer action");
    }
    fn usecase_action(&self, workdir: WorkDir, manifest: &'a Manifest) {
        println!("lang: {}", manifest.lang.name().as_str());
        println!("file path: {}", workdir.path);
        println!("usecase layer action");
    }
    fn presentation_action(&self, workdir: WorkDir, manifest: &'a Manifest) {
        println!("lang: {}", manifest.lang.name().as_str());
        println!("file path: {}", workdir.path);
        println!("presentation layer action");
    }
    fn gen_file_default(&self, workdir: WorkDir, manifest: &'a Manifest) {
        println!("lang: {}", manifest.lang.name().as_str());
        println!("file path: {}", workdir.path);
        println!("file genarate default action");
    }
}
