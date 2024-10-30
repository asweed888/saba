use crate::infrastructure::filesystem::manifest::ManifestRepository;
use crate::usecase::gen_file::codefile::act::CodefileAct;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use anyhow::anyhow;


pub struct Bash<'a> {
    repo: &'a ManifestRepository,
}

impl<'a> Bash<'a> {
    pub fn new(repo: &'a ManifestRepository) -> anyhow::Result<Self> {
        Ok(Self{ repo })
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.gen_location(&self.repo)?;
        Ok(())
    }
}

impl<'a> CodefileAct<'a> for Bash<'a> {
    fn gen_codefile_main(&self, wd: PathBuf, _repo: &'a ManifestRepository) -> anyhow::Result<()> {
        let path = wd.to_str().ok_or_else(|| anyhow!("Failed to convert wd to str type"))?;

        let mut file = File::create(path)?;
        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        file.set_permissions(permissions)?;
        file.write_all("#!/bin/bash".as_bytes())?;

        Ok(())
    }
}
