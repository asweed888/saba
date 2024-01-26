use sabacan::manifest::domain::model::entity::Manifest;
use sabacan::manifest::usecase::generate::codefile::CodefileGenerator;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

pub struct GenerateLuaFileUseCaseImpl {
    manifest: Manifest,
}

impl <'a> GenerateLuaFileUseCaseImpl {
    pub fn new(manifest: Manifest) -> Self {
        Self{ manifest }
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.location_action(&self.manifest)?;
        Ok(())
    }
}

impl<'a> CodefileGenerator<'a> for GenerateLuaFileUseCaseImpl {
    fn gen_file_default(
        &self,
        wd: PathBuf,
        _: &'a Manifest,
    ) -> anyhow::Result<()> {

        let mut file = File::create(wd.to_str().unwrap())?;
        file.write_all("return function()
end".as_bytes())?;

        Ok(())
    }
}