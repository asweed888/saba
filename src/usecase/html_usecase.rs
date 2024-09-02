use crate::domain::model::manifest::Manifest;
use crate::utils::act::codefile::Act as CodefileAct;
use std::path::PathBuf;
use std::fs::File;


pub struct Html<'a> {
    manifest: &'a Manifest,
}

impl<'a> Html<'a> {
    pub fn new(manifest: &'a mut Manifest) -> anyhow::Result<Self> {
        manifest.set_root(".");
        manifest.set_ext("html");

        Ok(Self{ manifest })
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.gen_location(&self.manifest)?;
        Ok(())
    }
}

impl<'a> CodefileAct<'a> for Html<'a> {
    fn gen_codefile_main(&self, wd: PathBuf, manifest: &'a Manifest) -> anyhow::Result<()> {
        let path = wd.to_str().unwrap();

        File::create(wd.to_str().unwrap())?;
        Ok(())
    }
}
