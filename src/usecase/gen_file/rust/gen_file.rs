use crate::infrastructure::filesystem::manifest::ManifestRepository;
use crate::usecase::gen_file::codefile::act::CodefileAct;
use super::modblock::ModBlock;
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::prelude::*;
use askama::Template;
use super::template::*;
use anyhow::anyhow;
use yaml_rust::Yaml;



pub struct Rust<'a> {
    repo: &'a ManifestRepository,
}

impl<'a> Rust<'a> {
    pub fn new(repo: &'a ManifestRepository) -> anyhow::Result<Self> {
        Ok(Self{ repo })
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.gen_location(&self.repo)?;
        Ok(())
    }
    fn gen_rustfile(
        &self,
        wd: PathBuf,
        codefile: &Vec<Yaml>,
        modblock: &mut ModBlock<'a>,
        repo: &'a ManifestRepository,
    ) -> anyhow::Result<()> {
        let ext = repo.manifest.lang.ext();
        for f in codefile {
            let mut workdir = wd.clone();
            let filename = f["name"].as_str().ok_or_else(|| anyhow!("Failed to get name from codefile"))?;
            let visibility = f["visibility"].as_str().unwrap_or("");
            modblock.update_body(filename, visibility)?;

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
    fn gen_rustdir(
        &self,
        wd: PathBuf,
        upstream: &Vec<Yaml>,
        modblock: &mut ModBlock<'a>,
        repo: &'a ManifestRepository,
    ) -> anyhow::Result<()> {
        let vec_default: &Vec<Yaml> = &vec![];

        for u in upstream {
            let mut workdir = wd.clone();
            let dirname = u["name"].as_str().ok_or_else(|| anyhow!("Failed to get name from upstream"))?;
            let upstream = u["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = u["codefile"].as_vec().unwrap_or(vec_default);
            let visibility = u["visibility"].as_str().unwrap_or("");
            modblock.update_body(dirname, visibility)?;

            workdir.push(dirname);
            fs::create_dir_all(workdir.clone())?;

            let mut modblock2 = ModBlock::new(workdir.clone(), &repo)?;
            if !codefile.is_empty() {
                self.gen_rustfile(workdir.clone(), codefile, &mut modblock2, &repo)?;
            }
            if !upstream.is_empty() {
                self.gen_rustdir(workdir.clone(), upstream, &mut modblock2, &repo)?;
            }
            modblock2.gen()?;
        }

        Ok(())
    }
}


impl<'a> CodefileAct<'a> for Rust<'a> {
    fn gen_location(&self, repo: &'a ManifestRepository) -> anyhow::Result<()> {
        let vec_default: &Vec<Yaml> = &vec![];
        let mut modblock = ModBlock::new(repo.manifest.root.clone(), repo)?;

        for spec in repo.manifest.spec.clone() {
            let mut workdir = repo.manifest.root.clone();
            let location = spec["location"].as_str().ok_or_else(|| anyhow!("Failed to get location from spec"))?;
            let upstream = spec["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = spec["codefile"].as_vec().unwrap_or(vec_default);
            let visibility = spec["visibility"].as_str().unwrap_or("");

            if location == "src" {
                if !codefile.is_empty() {
                    self.gen_rustfile(workdir.clone(), codefile, &mut modblock, &repo)?;
                }
            }
            else {
                modblock.update_body(location, visibility)?;
                workdir.push(location);
                fs::create_dir_all(workdir.clone())?;

                let mut modblock2 = ModBlock::new(workdir.clone(), &repo)?;
                if !codefile.is_empty() {
                    self.gen_rustfile(workdir.clone(), codefile, &mut modblock2, &repo)?;
                }
                if !upstream.is_empty() {
                    self.gen_rustdir(workdir.clone(), upstream, &mut modblock2, &repo)?;
                }
                modblock2.gen()?;
            }
        }
        modblock.gen()?;

        Ok(())
    }
    fn gen_codefile_main(&self, wd: PathBuf, repo: &'a ManifestRepository) -> anyhow::Result<()> {
        let path = wd.to_str().ok_or_else(|| anyhow!("Failed to convert wd to str type"))?;

        let is_ddd = repo.manifest.arch.is_ddd();
        let (fname, pkgname) = self.workdir_info(wd.clone(), &repo)?;
        let (fname, pkgname) = { (fname.as_str(), pkgname.as_str()) };

        if is_ddd {
            if path.contains("/domain/model/") {
                let data = DomainModelTmpl{fname, pkgname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(path)?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/domain/repository/") {
                let data = DomainRepositoryTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(path)?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/infrastructure/") {
                let data = InfraTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(path)?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/usecase/") {
                let data = UseCaseTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(path)?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else {
                let data = DefaultTmpl{fname, pkgname, wd: path};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(path)?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
        }
        else {
            let data = DefaultTmpl{fname, pkgname, wd: path};
            let rendered_tmpl = data.render()?;
            let mut file = File::create(path)?;
            file.write_all(rendered_tmpl.as_bytes())?;
        }
        Ok(())
    }
}
