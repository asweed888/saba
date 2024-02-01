use askama::Template;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use regex::Regex;
use yaml_rust::Yaml;
use sabacan::manifest::domain::model::entity::Manifest;
use sabacan::manifest::usecase::generate::codefile::{CodefileGenerator, PATH_LIST};
use sabacan::rust::modblock;
use crate::generate::codefile::rust::template::{
    DomainModelTmpl,
    DomainRepositoryTmpl,
    InfraTmpl,
    UseCaseTmpl,
    PresentationTmpl,
    DiTmpl,
    DefaultTmpl,
};

pub struct GenerateRustFileUseCaseImpl {
    manifest: Manifest,
}

impl<'a> GenerateRustFileUseCaseImpl {
    pub fn new(manifest: Manifest) -> Self {
        Self{ manifest }
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.location_action(&self.manifest)?;
        Ok(())
    }
}

impl<'a> modblock::Handler<'a> for GenerateRustFileUseCaseImpl {
    fn write_modblock(&self, manifest: &'a Manifest) -> anyhow::Result<()> {
        let main_rs_path = self.main_rs_path(&manifest, None)?;
        Ok(())
    }
    fn main_rs_path(&self, manifest: Option<&'a Manifest>, path: Option<&'a PathBuf>) {

    }
}

impl<'a> CodefileGenerator<'a> for GenerateRustFileUseCaseImpl {
    fn di_container_action(
        &self,
        wd: PathBuf,
        _: &'a Manifest,
    ) -> anyhow::Result<()> {
        let path_list_raw = PATH_LIST.lock().unwrap();
        let path_list = &*path_list_raw;
        let data = DiTmpl{
            imports: path_list,
        };

        let rendered_tmpl = data.render()?;

        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
    fn domain_model_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {

        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let data = DomainModelTmpl{
            fname: fname.as_str(),
            pkgname: pkgname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
    fn domain_repository_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = DomainRepositoryTmpl{
            fname: fname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
    fn infra_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = InfraTmpl{
            fname: fname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
    fn usecase_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = UseCaseTmpl{
            fname: fname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
    fn presentation_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let data = PresentationTmpl{
            fname: fname.as_str(),
            pkgname: pkgname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
    fn gen_file_default(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let data = DefaultTmpl{
            fname: fname.as_str(),
            pkgname: pkgname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
}