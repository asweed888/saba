use crate::domain::manifest::entity::Manifest;
use crate::usecase::manifest::interface::TGenerateFileUseCase;
use askama::Template;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use anyhow::Result;
use crate::usecase::manifest::golang::template::{
    DomainModelTmpl,
    DomainRepositoryTmpl,
    InfraTmpl,
    UseCaseTmpl,
    PresentationTmpl,
    DefaultTmpl,
    di_tmpl,
};


pub struct GoLangUseCase {
    manifest: Manifest,
}

impl<'a> GoLangUseCase {
    pub fn new(manifest: Manifest) -> Self {
        Self{ manifest }
    }
    pub fn gen_file(&self) -> Result<()> {
        self.location_action(&self.manifest)?;
        Ok(())
    }
}


impl<'a> TGenerateFileUseCase<'a> for GoLangUseCase {
    fn di_container_action(
        &self,
        wd: PathBuf,
        _: &'a Manifest,
    ) -> Result<()> {
        let data = di_tmpl();

        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(data.as_bytes())?;

        Ok(())
    }
    fn domain_model_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = DomainModelTmpl{
            pkgname: pkgname.as_str(),
            fname: fname.as_str(),
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
    ) -> Result<()> {
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = DomainRepositoryTmpl{
            pkgname: pkgname.as_str(),
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
    ) -> Result<()> {
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = InfraTmpl{
            pkgname: pkgname.as_str(),
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
    ) -> Result<()> {
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = UseCaseTmpl{
            pkgname: pkgname.as_str(),
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
    ) -> Result<()> {
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = PresentationTmpl{
            pkgname: pkgname.as_str(),
            fname: fname.as_str(),
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
    ) -> Result<()> {
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let data = DefaultTmpl{
            pkgname: pkgname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
}
