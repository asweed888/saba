use sabacan::manifest::domain::model::entity::Manifest;
use sabacan::manifest::usecase::generate::codefile::CodefileGenerator;
use askama::Template;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use crate::usecase::generate::codefile::typescript::template::{
    DomainModelTmpl,
    DomainRepositoryTmpl,
    InfraTmpl,
    UseCaseTmpl,
    PresentationTmpl,
    DefaultTmpl,
    di_tmpl,
    vue_tmpl,
};


pub struct GenerateTypeScriptFileUseCaseImpl {
    manifest: Manifest,
}

impl<'a> GenerateTypeScriptFileUseCaseImpl {
    pub fn new(manifest: Manifest) -> Self {
        Self{ manifest }
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.location_action(&self.manifest)?;
        Ok(())
    }
    fn gen_irregular_file_terminus(
        &self,
        wd: PathBuf,
    ) -> anyhow::Result<bool> {
        if wd.extension().unwrap() == "vue" {
            let data = vue_tmpl();
            let mut file = File::create(wd.to_str().unwrap())?;

            file.write_all(data.as_bytes())?;
            return Ok(true)
        }
        Ok(false)
    }
}


impl<'a> CodefileGenerator<'a> for GenerateTypeScriptFileUseCaseImpl {
    fn di_container_action(
        &self,
        wd: PathBuf,
        _: &'a Manifest,
    ) -> anyhow::Result<()> {
        let data = di_tmpl();

        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(data.as_bytes())?;

        Ok(())
    }
    fn domain_model_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = DomainModelTmpl{
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
    ) -> anyhow::Result<()> {

        // .vueの場合別のテンプレートでファイルを出力
        if self.gen_irregular_file_terminus(wd.clone()).unwrap() {
            return Ok(())
        }

        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = DefaultTmpl{
            fname: fname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
}
