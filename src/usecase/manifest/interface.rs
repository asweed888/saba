use yaml_rust::Yaml;
use crate::domain::manifest::entity::Manifest;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use anyhow::Result;

lazy_static::lazy_static! {
    pub static ref PATH_LIST: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}

pub trait TGenerateFileUseCase<'a> {
    fn location_action(&self, manifest: &'a Manifest) -> Result<()> {
        let root_path = manifest.root.get_path();
        let vec_default: &Vec<Yaml> = &vec![];

        for spec in manifest.spec.clone() {
            let mut workdir = PathBuf::from(&root_path);
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = spec["codefile"].as_vec().unwrap_or(vec_default);

            workdir.push(location);
            fs::create_dir_all(workdir.clone())?;

            if !upstream.is_empty() {
                self.upstream_action(workdir.clone(), upstream, &manifest)?;
            }

            if !codefile.is_empty() {
                self.codefile_action(workdir.clone(), codefile, &manifest)?;
            }
        }
        Ok(())
    }
    fn upstream_action(
        &self,
        wd: PathBuf,
        upstream: &Vec<Yaml>,
        manifest: &'a Manifest
    ) -> Result<()> {
        let vec_default: &Vec<Yaml> = &vec![];

        for u in upstream {
            let mut workdir = PathBuf::from(wd.to_str().unwrap());
            let dirname = u["name"].as_str().unwrap();
            let upstream = u["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = u["codefile"].as_vec().unwrap_or(vec_default);

            workdir.push(dirname);
            fs::create_dir_all(workdir.clone())?;

            if !upstream.is_empty() {
                self.upstream_action(workdir.clone(), upstream, manifest)?;
            }

            if !codefile.is_empty() {
                self.codefile_action(workdir.clone(), codefile, manifest)?;
            }
        }
        Ok(())
    }
    fn codefile_action(
        &self,
        wd: PathBuf,
        codefile: &Vec<Yaml>,
        manifest: &'a Manifest
    ) -> Result<()> {
        let ext = manifest.lang.ext().as_str();
        let mut is_ddd = false;
        let mut is_di_container = false;
        let mut di_path: PathBuf = PathBuf::from("");

        for f in codefile {
            let mut workdir = PathBuf::from(wd.to_str().unwrap());
            let filename = f["name"].as_str().unwrap();

            workdir.push(filename);
            workdir.set_extension(ext);
            let path = workdir.to_str().unwrap();
            let exists = workdir.as_path().exists();
            is_ddd = self.is_ddd_enabled(manifest);

            if is_ddd {
                if !exists && path.contains("/domain/model/") {
                    self.domain_model_action(workdir.clone(), manifest)?;
                }
                else if !exists && path.contains("/domain/repository/") {
                    self.domain_repository_action(workdir.clone(), manifest)?;
                }
                else if !exists && path.contains("/infrastructure/") {
                    self.infra_action(workdir.clone(), manifest)?;
                }
                else if !exists && path.contains("/usecase/") {
                    self.usecase_action(workdir.clone(), manifest)?;
                }
                else if !exists && path.contains("/presentation/") {
                    self.presentation_action(workdir.clone(), manifest)?;
                }
                else if path.contains("/di/") {
                    is_di_container = true;
                    di_path = workdir.clone();
                }
                else if !exists {
                    self.gen_file_default_ddd(workdir.clone(), manifest)?;
                }

                self.save_path(workdir.clone(), manifest)?;
            }
            else if !exists {
                self.gen_file_default(workdir.clone(), manifest)?;
            }
        }

        if !di_path.as_path().exists() && is_ddd && is_di_container {
            self.di_container_action(di_path.clone(), manifest)?;
        }
        Ok(())
    }
    fn is_ddd_enabled(&self, manifest: &'a Manifest) -> bool {
        manifest.arch.is_ddd()
        && manifest.lang.name().as_str() != "bash"
    }
    fn save_path(
        &self,
        mut wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        wd.set_extension("");
        let mut path_list = PATH_LIST.lock().unwrap();
        let root = manifest.root.get_path();
        let path = wd.to_str()
            .unwrap()
            .replace(root.as_str(), "");
        path_list.push(path);
        Ok(())
    }
    fn di_container_action(
        &self,
        _: PathBuf,
        _: &'a Manifest,
    ) -> Result<()> {
        let path_list = PATH_LIST.lock().unwrap();
        for path in path_list.iter() {
            println!("path is {}", path);
        }
        Ok(())
    }
    fn domain_model_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.generated_file_message(wd.clone(), manifest)
    }
    fn domain_repository_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.generated_file_message(wd.clone(), manifest)
    }
    fn infra_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.generated_file_message(wd.clone(), manifest)
    }
    fn usecase_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.generated_file_message(wd.clone(), manifest)
    }
    fn presentation_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.generated_file_message(wd.clone(), manifest)
    }
    fn gen_file_default_ddd(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default(wd.clone(), manifest)
    }
    fn gen_file_default(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.generated_file_message(wd.clone(), manifest)
    }
    fn generated_file_message(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        println!("------------");
        println!("generate {} on {}", fname, pkgname);
        println!("file path: {}", wd.to_str().unwrap());
        println!("------------");
        println!("");
        Ok(())
    }
    fn get_fname(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Option<String> {
        let root = manifest.root.get_path();
        Some(
            wd.file_stem()
            .unwrap()
            .to_str()
            .unwrap_or(root.as_str())
            .to_string()
        )
    }
    fn get_pkgname(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Option<String> {
        let root = manifest.root.get_path();
        let parent = wd.parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or("");

        match root.as_str() {
            "." => {
                if parent != "." {
                    Some(parent.to_string())
                }
                else {
                    None
                }
            }
            _ => {
                let replaced = root.replace("./", "");
                if parent != replaced.as_str() {
                    Some(parent.to_string())
                }
                else {
                    None
                }
            }
        }
    }
}
