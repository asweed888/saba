use crate::domain::model::manifest::Manifest;
use crate::infrastructure::filesystem::manifest::ManifestRepository;
use crate::usecase::gen_file::codefile::act::CodefileAct;
use super::utils as rs_utils;
use super::modblock::ModBlock;
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::prelude::*;
use regex::Regex;
use askama::Template;
use super::template::*;
use anyhow::anyhow;
use yaml_rust::Yaml;



pub struct Rust<'a> {
    repo: &'a ManifestRepository,
    modblock: ModBlock<'a>,
}

impl<'a> Rust<'a> {
    pub fn new(repo: &'a mut ManifestRepository) -> anyhow::Result<Self> {
        Ok(Self{
            repo,
            modblock: ModBlock::new(repo.manifest.root.clone(), &repo)?,
        })
    }
    fn gen_location(&self, repo: &'a ManifestRepository) -> anyhow::Result<()> {
        let mut modblock: ModBlock<'a>;

        for spec in repo.manifest.spec.clone() {
            let mut workdir = repo.manifest.root.clone();
            let location = spec["location"].as_str().ok_or_else(|| anyhow!("Failed to get location from spec"))?;
            let upstream = spec["upstream"].as_vec().unwrap_or(&vec![]);
            let codefile = spec["codefile"].as_vec().unwrap_or(&vec![]);
            let visibility = spec["visibility"].as_str().unwrap_or("");
            self.modblock.update_body(location, visibility)?;
            modblock = self.modblock.clone();

            if location != "src" {
                workdir.push(location);
                fs::create_dir_all(workdir.clone())?;
            }

            if !codefile.is_empty() {
                self.gen_codefile(workdir.clone(), codefile, &repo)?;
            }
            if !upstream.is_empty() {
                self.gen_upstream(workdir.clone(), upstream, &repo)?;
            }
        }
        modblock.gen()?;

        Ok(())
    }
    fn gen_upstream(&self, wd: PathBuf, upstream: &Vec<Yaml>, repo: &'a ManifestRepository) -> anyhow::Result<()> {
        self.modblock = ModBlock::new(wd.clone(), &repo)?;
        let mut modblock: ModBlock<'a>;

        for u in upstream {
            let mut workdir = wd.clone();
            let dirname = u["name"].as_str().ok_or_else(|| anyhow!("Failed to get name from upstream"))?;
            let upstream = u["upstream"].as_vec().unwrap_or(&vec![]);
            let codefile = u["codefile"].as_vec().unwrap_or(&vec![]);
            let visibility = u["visibility"].as_str().unwrap_or("");
            self.modblock.update_body(dirname, visibility)?;

            workdir.push(dirname);
            fs::create_dir_all(workdir.clone())?;

            if !codefile.is_empty() {
                self.gen_codefile(workdir.clone(), codefile, &repo)?;
                modblock = self.modblock.clone();
            }
            if !upstream.is_empty() {
                self.gen_upstream(workdir.clone(), upstream, &repo)?;
            }
        }
        modblock.gen()?;

        Ok(())
    }
    fn gen_codefile(&self, wd: PathBuf, codefile: &Vec<Yaml>, repo: &'a ManifestRepository) -> anyhow::Result<()> {
        let ext = repo.manifest.lang.ext();
        for f in codefile {
            let mut workdir = wd.clone();
            let filename = f["name"].as_str().ok_or_else(|| anyhow!("Failed to get name from codefile"))?;
            let visibility = f["visibility"].as_str().unwrap_or("");
            self.modblock.update_body(filename, visibility)?;

            if repo.manifest.lang.is_generate_ignore(filename) {
                continue;
            }

            workdir.push(filename);
            self.set_ext(&mut workdir, ext)?;
            if !workdir.as_path().exists() {
                self.gen_codefile_main(workdir.clone(), &repo)?;
            }
        }

        Ok(())
    }
}

