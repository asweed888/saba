use yaml_rust::Yaml;
use crate::domain::manifest::entity::Manifest;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use anyhow::Result;

lazy_static::lazy_static! {
    static ref PATH_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());
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

        for f in codefile {
            let mut workdir = PathBuf::from(wd.to_str().unwrap());
            let filename = f["name"].as_str().unwrap();

            workdir.push(filename);
            workdir.set_extension(ext);
            let path = workdir.to_str().unwrap();

            if self.is_ddd_enabled(manifest) {
                if path.contains("/domain/model/") {
                    self.domain_model_action(workdir.clone(), manifest)?;
                }
                else if path.contains("/domain/repository/") {
                    self.domain_repository_action(workdir.clone(), manifest)?;
                }
                else if path.contains("/infrastructure/") {
                    self.infra_action(workdir.clone(), manifest)?;
                }
                else if path.contains("/usecase/") {
                    self.usecase_action(workdir.clone(), manifest)?;
                }
                else if path.contains("/presentation/") {
                    self.presentation_action(workdir.clone(), manifest)?;
                }
                else if !path.contains("/di/") {
                    self.gen_file_default_ddd(workdir.clone(), manifest)?;
                }

                println!("path is: {}", path);
            }
        }
        Ok(())
    }
    fn is_ddd_enabled(&self, manifest: &'a Manifest) -> bool {
        manifest.arch.is_ddd()
        && manifest.lang.name().as_str() != "bash"
    }
    fn save_path(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        let root = manifest.root.get_path().as_str();

        Ok(())
    }
    fn di_container_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default_ddd(wd.clone(), manifest)
    }
    fn domain_model_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default_ddd(wd.clone(), manifest)
    }
    fn domain_repository_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default_ddd(wd.clone(), manifest)
    }
    fn infra_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default_ddd(wd.clone(), manifest)
    }
    fn usecase_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default_ddd(wd.clone(), manifest)
    }
    fn presentation_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default_ddd(wd.clone(), manifest)
    }
    fn gen_file_default_ddd(
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
