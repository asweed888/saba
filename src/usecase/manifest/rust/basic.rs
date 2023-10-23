use crate::domain::manifest::entity::Manifest;
use crate::usecase::manifest::interface::WorkDir;
use crate::usecase::manifest::interface::TGenerateFileUseCase;
use crate::usecase::manifest::rust::template::{
    DomainModelTmpl,
    DomainRepositoryTmpl,
    InfraTmpl,
    UseCaseTmpl,
    PresentationTmpl,
};
use askama::Template;
use std::fs::File;
use std::io::prelude::*;
use anyhow::Result;

#[derive(Debug)]
pub struct RustUseCase {
    manifest: Manifest,
}

impl<'a> RustUseCase {
    pub fn new(manifest: Manifest) -> Self {
        Self{
            manifest,
        }
    }
    pub fn gen_file(&self) -> Result<()> {
        self.location_action(&self.manifest)?;
        Ok(())
    }
}

impl<'a> TGenerateFileUseCase<'a> for RustUseCase {}
// impl<'a> TGenerateFileUseCase<'a> for RustUseCase<'a> {
//     fn domain_model_action(
//         &self,
//         mut workdir: WorkDir,
//     ) -> Result<()> {
//
//         let fname = workdir.fname().unwrap();
//         let data = DomainModelTmpl{
//             fname: fname.as_str(),
//         };
//
//         let rendered_tmpl = data.render()?;
//         let mut file = File::create(workdir.path)?;
//
//         file.write_all(rendered_tmpl.as_bytes())?;
//
//         Ok(())
//     }
//     fn domain_repository_action(
//         &self,
//         mut workdir: WorkDir,
//     ) -> Result<()> {
//         let fname = workdir.fname().unwrap();
//         let data = DomainRepositoryTmpl{
//             fname: fname.as_str(),
//         };
//
//         let rendered_tmpl = data.render()?;
//         let mut file = File::create(workdir.path)?;
//
//         file.write_all(rendered_tmpl.as_bytes())?;
//
//         Ok(())
//     }
//     fn infra_action(
//         &self,
//         mut workdir: WorkDir,
//     ) -> Result<()> {
//         let fname = workdir.fname().unwrap();
//         let data = InfraTmpl{
//             fname: fname.as_str(),
//         };
//
//         let rendered_tmpl = data.render()?;
//         let mut file = File::create(workdir.path)?;
//
//         file.write_all(rendered_tmpl.as_bytes())?;
//
//         Ok(())
//     }
//     fn usecase_action(
//         &self,
//         mut workdir: WorkDir,
//     ) -> Result<()> {
//         let fname = workdir.fname().unwrap();
//         let data = UseCaseTmpl{
//             fname: fname.as_str(),
//         };
//
//         let rendered_tmpl = data.render()?;
//         let mut file = File::create(workdir.path)?;
//
//         file.write_all(rendered_tmpl.as_bytes())?;
//
//         Ok(())
//     }
//     fn presentation_action(
//         &self,
//         mut workdir: WorkDir,
//     ) -> Result<()> {
//         let fname = workdir.fname().unwrap();
//         let pkgname = workdir.pkgname().unwrap();
//         let data = PresentationTmpl{
//             fname: fname.as_str(),
//             pkgname: pkgname.as_str(),
//         };
//
//         let rendered_tmpl = data.render()?;
//         let mut file = File::create(workdir.path)?;
//
//         file.write_all(rendered_tmpl.as_bytes())?;
//
//         Ok(())
//     }
//     fn gen_file_default(
//         &self,
//         workdir: WorkDir,
//     ) -> Result<()> {
//
//         let mut file = File::create(workdir.path)?;
//         let file_contents = String::from("");
//
//         file.write_all(file_contents.as_bytes())?;
//
//         Ok(())
//     }
// }
