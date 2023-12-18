use domain::model::manifest::entity::Manifest;
use crate::manifest::interface::TGenerateFileUseCase;
use anyhow::Result;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

pub struct LuaUseCase {
    manifest: Manifest,
}

impl <'a> LuaUseCase {
    pub fn new(manifest: Manifest) -> Self {
        Self{ manifest }
    }
    pub fn gen_file(&self) -> Result<()> {
        self.location_action(&self.manifest)?;
        Ok(())
    }
}

impl<'a> TGenerateFileUseCase<'a> for LuaUseCase {
    fn gen_file_default(
        &self,
        wd: PathBuf,
        _: &'a Manifest,
    ) -> Result<()> {

        let mut file = File::create(wd.to_str().unwrap())?;
        file.write_all("return function()
end".as_bytes())?;

        Ok(())
    }
}