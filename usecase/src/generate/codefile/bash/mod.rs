use sabacan::manifest::domain::model::entity::Manifest;
use sabacan::manifest::usecase::generate::codefile::CodefileGenerator;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;

pub struct GenerateBashFileUseCaseImpl {
    manifest: Manifest,
}

impl<'a> GenerateBashFileUseCaseImpl {
    pub fn new(manifest: Manifest) -> Self {
        Self{ manifest }
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.location_action(&self.manifest)?;
        Ok(())
    }
}


impl<'a> CodefileGenerator<'a> for GenerateBashFileUseCaseImpl {
    fn gen_file_default(
        &self,
        wd: PathBuf,
        _: &'a Manifest,
    ) -> anyhow::Result<()> {

        let mut file = File::create(wd.to_str().unwrap())?;
        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        file.set_permissions(permissions)?;
        file.write_all("#!/bin/bash".as_bytes())?;

        Ok(())
    }
}