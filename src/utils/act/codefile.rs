use crate::domain::model::manifest::{Manifest, MANIFEST};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use yaml_rust::Yaml;


pub trait Act {
    fn gen_location(&self) -> anyhow::Result<()> {
        let manifest: Manifest;
        {
            manifest = MANIFEST.lock().unwrap().clone();
        }
        let root_path = manifest.root.clone();
        let vec_default: &Vec<Yaml> = &vec![];

        for spec in manifest.spec.clone() {
            let mut workdir = PathBuf::from(&root_path);
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = spec["codefile"].as_vec().unwrap_or(vec_default);

            if location != "src" {
                workdir.push(location);
                fs::create_dir_all(workdir.clone())?;
            }

            if !upstream.is_empty() {
                self.gen_upstream(workdir.clone(), upstream)?;
            }

            if !codefile.is_empty() {
                self.gen_codefile(workdir.clone(), codefile)?;
            }
        }

        self.gen_location_post()?;
        Ok(())
    }
    fn gen_upstream(&self, wd: PathBuf, upstream: &Vec<Yaml>) -> anyhow::Result<()> {
        let vec_default: &Vec<Yaml> = &vec![];

        for u in upstream {
            let mut workdir = PathBuf::from(wd.to_str().unwrap());
            let dirname = u["name"].as_str().unwrap();
            let upstream = u["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = u["codefile"].as_vec().unwrap_or(vec_default);


            workdir.push(dirname);
            fs::create_dir_all(workdir.clone())?;

            if !upstream.is_empty() {
                self.gen_upstream(workdir.clone(), upstream)?;
            }

            if !codefile.is_empty() {
                self.gen_codefile(workdir.clone(), codefile)?;
            }
        }

        self.gen_upstream_post(wd.clone())?;
        Ok(())
    }
    fn gen_codefile(&self, wd: PathBuf, codefile: &Vec<Yaml>) -> anyhow::Result<()> {
        let manifest: Manifest;
        {
            manifest = MANIFEST.lock().unwrap().clone();
        }
        let ext = manifest.ext.clone();

        for f in codefile {
            let mut workdir = PathBuf::from(wd.to_str().unwrap());
            let filename = f["name"].as_str().unwrap();

            if filename == manifest.main_file.as_str()
                || filename == manifest.mod_file.as_str()
            {
                continue;
            }

            workdir.push(filename);
            self.set_ext(&mut workdir, ext.clone())?;

            if !workdir.as_path().exists() {
                self.gen_codefile_main(workdir.clone())?
            }
        }

        Ok(())
    }
    fn gen_codefile_main(&self, wd: PathBuf) -> anyhow::Result<()> {
        File::create(wd.to_str().unwrap())?;
        Ok(())
    }
    fn set_ext(&self, wd: &mut PathBuf, ext: String) -> anyhow::Result<()> {
        let ext = ext.as_str();
        if wd.to_str().unwrap().contains(".svelte") {
            wd.set_extension("svelte");
        }
        else if wd.to_str().unwrap().contains(".tsx") {
            wd.set_extension("tsx");
        }
        else if wd.to_str().unwrap().contains(".vue") {
            wd.set_extension("vue");
        }
        else {
            wd.set_extension(ext);
        }
        Ok(())
    }
    fn gen_location_post(&self) -> anyhow::Result<()> {
        Ok(())
    }
    fn gen_upstream_post(&self, _wd: PathBuf) -> anyhow::Result<()> {
        Ok(())
    }
    fn workdir_info(&self, wd: PathBuf) -> (Option<String>, Option<String>) {
        let manifest: Manifest;
        {
            manifest = MANIFEST.lock().unwrap().clone();
        }
        let root = manifest.root.clone();
        let fname = Some(
            wd.file_stem()
            .unwrap()
            .to_str()
            .unwrap_or(root.as_str())
            .to_string()
        );

        let parent = wd
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or("");

        let pkgname = match root.as_str() {
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
        };

        (fname, pkgname)
    }
}