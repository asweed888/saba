use crate::domain::manifest::entity::Manifest;
use crate::usecase::manifest::interface::WorkDir;
use crate::usecase::manifest::interface::TGenerateFileUseCase;
use crate::usecase::manifest::rust::template::{
    DomainModelTmpl,
    DomainRepositoryTmpl,
    InfraTmpl,
    UseCaseTmpl,
    PresentationTmpl,
    DiTmpl,
};
use askama::Template;
use std::fs::File;


pub struct RustUseCase<'a> {
    manifest: Manifest<'a>,
}

impl<'a> RustUseCase<'a> {
    pub fn new(manifest: Manifest<'a>) -> Self {
        Self{
            manifest,
        }
    }
    pub fn gen_file(&self){
        self.location_action(&self.manifest);
    }
}

impl<'a> TGenerateFileUseCase<'a> for RustUseCase<'a> {
    fn domain_model_action(
        &self,
        mut workdir: WorkDir,
        manifest: &'a Manifest
    ) -> Result<(), Box<dyn std::error::Error>> {

        let fname = workdir.fname().unwrap();
        let data = DomainModelTmpl{
            fname: fname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        Ok(())
    }
}
