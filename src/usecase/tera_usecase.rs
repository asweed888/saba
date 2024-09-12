use crate::domain::model::manifest::Manifest;
use crate::utils::act::codefile::Act as CodefileAct;
use std::path::PathBuf;
use std::fs::File;


pub struct Tera<'a> {
    manifest: &'a Manifest,
}

impl<'a> Tera<'a> {
    pub fn new(manifest: &'a mut Manifest) -> anyhow::Result<Self> {
        manifest.set_root(".");
        manifest.set_ext("html.tera");

        Ok(Self{ manifest })
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.gen_location(&self.manifest)?;
        Ok(())
    }
}

impl<'a> CodefileAct<'a> for Tera<'a> {
    fn gen_codefile_main(&self, wd: PathBuf, _manifest: &'a Manifest) -> anyhow::Result<()> {

        File::create(wd.to_str().unwrap())?;
        Ok(())
    }
}
