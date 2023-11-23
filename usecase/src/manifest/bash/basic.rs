use domain::model::manifest::entity::Manifest;
use crate::manifest::interface::TGenerateFileUseCase;
use anyhow::Result;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;

pub struct BashUseCase {
    manifest: Manifest,
}

impl<'a> BashUseCase {
    pub fn new(manifest: Manifest) -> Self {
        Self{ manifest }
    }
    pub fn gen_file(&self) -> Result<()> {
        self.location_action(&self.manifest)?;
        Ok(())
    }
}


impl<'a> TGenerateFileUseCase<'a> for BashUseCase {
    fn gen_file_default(
        &self,
        wd: PathBuf,
        _: &'a Manifest,
    ) -> Result<()> {

        let mut file = File::create(wd.to_str().unwrap())?;
        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        file.set_permissions(permissions)?;
        file.write_all("#!/bin/bash".as_bytes())?;

        Ok(())
    }
    fn di_container_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default(wd.clone(), manifest)?;
        Ok(())
    }
    fn domain_model_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default(wd.clone(), manifest)?;
        Ok(())
    }
    fn domain_repository_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default(wd.clone(), manifest)?;
        Ok(())
    }
    fn infra_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default(wd.clone(), manifest)?;
        Ok(())
    }
    fn usecase_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default(wd.clone(), manifest)?;
        Ok(())
    }
    fn presentation_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> Result<()> {
        self.gen_file_default(wd.clone(), manifest)?;
        Ok(())
    }
}