use crate::domain::model::manifest::Manifest;
use crate::utils::act::codefile::Act as CodefileAct;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;


pub struct Lua<'a> {
    manifest: &'a Manifest,
}

impl<'a> Lua<'a> {
    pub fn new(manifest: &'a mut Manifest) -> anyhow::Result<Self> {
        manifest.set_root(".");
        manifest.set_ext("lua");

        Ok(Self{ manifest })
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.gen_location(&self.manifest)?;
        Ok(())
    }
}

impl<'a> CodefileAct<'a> for Lua<'a> {
    fn gen_codefile_main(&self, wd: PathBuf, _manifest: &'a Manifest) -> anyhow::Result<()> {

        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all("return function()
end".as_bytes())?;

        Ok(())
    }
}
