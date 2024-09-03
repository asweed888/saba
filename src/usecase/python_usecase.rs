use crate::domain::model::manifest::Manifest;
use crate::utils::act::codefile::Act as CodefileAct;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use askama::Template;
use crate::utils::templates::python::{
    DomainModelTmpl,
    DomainRepositoryTmpl,
    InfraTmpl,
    UseCaseTmpl,
    PresentationTmpl,
    DefaultTmpl,
};

pub struct Python<'a> {
    manifest: &'a Manifest,
}

impl<'a> Python<'a> {
    pub fn new(manifest: &'a mut Manifest) -> anyhow::Result<Self> {
        manifest.set_root(".");
        manifest.set_ext("py");

        Ok(Self{ manifest })
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.gen_location(&self.manifest)?;
        Ok(())
    }
}

impl<'a> CodefileAct<'a> for Python<'a> {
    fn gen_codefile_main(&self, wd: PathBuf, manifest: &'a Manifest) -> anyhow::Result<()> {
        let path = wd.to_str().unwrap();

        let is_ddd = manifest.is_ddd();
        let (fname, pkgname) = self.workdir_info(wd.clone(), &manifest);
        let (fname, pkgname) = { (fname.unwrap(), pkgname.unwrap()) };
        let (fname, pkgname) = { (fname.as_str(), pkgname.as_str()) };

        if is_ddd {
            if path.contains("/domain/model/") {
                let data = DomainModelTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(wd.to_str().unwrap())?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/domain/repository/") {
                let data = DomainRepositoryTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(wd.to_str().unwrap())?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/infrastructure/") {
                let data = InfraTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(wd.to_str().unwrap())?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/usecase/") {
                let data = UseCaseTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(wd.to_str().unwrap())?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/presentation/") {
                let data = PresentationTmpl{fname, pkgname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(wd.to_str().unwrap())?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else {
                let data = DefaultTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(wd.to_str().unwrap())?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
        }
        else {
            let data = DefaultTmpl{fname};
            let rendered_tmpl = data.render()?;
            let mut file = File::create(wd.to_str().unwrap())?;
            file.write_all(rendered_tmpl.as_bytes())?;
        }

        Ok(())
    }
}
