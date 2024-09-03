use crate::domain::model::manifest::Manifest;
use crate::utils::act::codefile::Act as CodefileAct;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;


pub struct Bash<'a> {
    manifest: &'a Manifest,
}

impl<'a> Bash<'a> {
    pub fn new(manifest: &'a mut Manifest) -> anyhow::Result<Self> {
        manifest.set_root(".");
        manifest.set_ext("");

        Ok(Self{ manifest })
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.gen_location(&self.manifest)?;
        Ok(())
    }
}

impl<'a> CodefileAct<'a> for Bash<'a> {
    fn gen_codefile_main(&self, wd: PathBuf, _manifest: &'a Manifest) -> anyhow::Result<()> {

        let mut file = File::create(wd.to_str().unwrap())?;
        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        file.set_permissions(permissions)?;
        file.write_all("#!/bin/bash".as_bytes())?;

        Ok(())
    }
}
