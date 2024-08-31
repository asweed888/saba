use crate::domain::model::manifest::MANIFEST;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use yaml_rust::Yaml;


pub trait Act {
    fn gen_location(&self) -> anyhow::Result<()> {
        let manifest = MANIFEST.lock().unwrap();
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
        }

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
        }

        Ok(())
    }
    fn gen_codefile(&self, wd: PathBuf, codefile: &Vec<Yaml>) -> anyhow::Result<()> {
        let manifest = MANIFEST.lock().unwrap();
        let ext = manifest.ext.clone();

        for f in codefile {
            let mut workdir = PathBuf::from(wd.to_str().unwrap());
            let filename = f["name"].as_str().unwrap();

            workdir.push(filename);
            self.set_ext(&mut workdir, ext.as_str())?;

            if !workdir.as_path().exists() {
                self.gen_codefile_default(workdir.clone())?
            }
        }

        Ok(())
    }
    fn gen_codefile_default(&self, wd: PathBuf) -> anyhow::Result<()> {
        File::create(wd.to_str().unwrap())?;
        Ok(())
    }
    fn set_ext(&self, wd: &mut PathBuf, ext: &'a str) -> anyhow::Result<()> {
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
}