use sabacan::manifest::domain::model::entity::Manifest;
use sabacan::manifest::usecase::generate::codefile::CodefileGenerator;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;

pub struct GenerateHtmlFileUseCaseImpl {
    manifest: Manifest,
}

impl<'a> GenerateHtmlFileUseCaseImpl {
    pub fn new(manifest: Manifest) -> Self {
        Self{ manifest }
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.location_action(&self.manifest)?;
        Ok(())
    }
}


impl<'a> CodefileGenerator<'a> for GenerateHtmlFileUseCaseImpl {
    fn gen_file_default(
        &self,
        wd: PathBuf,
        _: &'a Manifest,
    ) -> anyhow::Result<()> {

        let mut file = File::create(wd.to_str().unwrap())?;

        Ok(())
    }
}
