use clap::Command;
use anyhow::{bail, Result};
use crate::usecase::manifest::basic::ManifestUseCase;
use crate::usecase::manifest::rust::basic::RustUseCase;
use crate::usecase::manifest::golang::basic::GoLangUseCase;


pub struct UpCommand {
    pub manifest: ManifestUseCase,
}

impl UpCommand {
    pub fn new(manifest: ManifestUseCase) -> Self {
        Self{ manifest }
    }
    pub fn spec(&self) -> Command {
        Command::new("up")
            .about("up command")
    }
    pub fn action(&self) -> Result<()> {
        let mut manifest = self.manifest.load().unwrap();

        match manifest.lang.name().as_str() {
            "rust" => {
                manifest.lang.set_ext(String::from("rs"));
                manifest.root.set_default(String::from("./src"));
                let uc = RustUseCase::new(manifest);
                uc.gen_file()?;
            }
            "go" => {
                manifest.lang.set_ext(String::from("go"));
                manifest.root.set_default(String::from("."));
                let uc = GoLangUseCase::new(manifest);
                uc.gen_file()?;
            }
            _ => {
                bail!("[ERROR] The language is not supported.")
            }
        }
        println!("[Success] generate of saba has been completed.");
        Ok(())
    }
}