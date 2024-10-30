use crate::infrastructure::filesystem::manifest::ManifestRepository;
use crate::usecase::gen_file::codefile::act::CodefileAct;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use anyhow::anyhow;

pub struct Lua<'a> {
    repo: &'a ManifestRepository,
}

impl<'a> Lua<'a> {
    pub fn new(repo: &'a ManifestRepository) -> anyhow::Result<Self> {
        Ok(Self{ repo })
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.gen_location(&self.repo)?;
        Ok(())
    }
}


impl<'a> CodefileAct<'a> for Lua<'a> {
    fn gen_codefile_main(&self, wd: PathBuf, _repo: &'a ManifestRepository) -> anyhow::Result<()> {
        let path = wd.to_str().ok_or_else(|| anyhow!("Failed to convert wd to str type"))?;

        let mut file = File::create(path)?;

        file.write_all("return function()
end".as_bytes())?;
        Ok(())
    }
}
