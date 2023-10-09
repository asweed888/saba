use crate::usecase::manifest::ManifestUseCase;
use crate::domain::repository::code_file::CodeFileRepository;
use crate::usecase::rust_file::RustFileUseCase;
use crate::usecase::go_file::GoFileUseCase;
use clap::Command;

pub struct UpCommand {
    pub manifest: ManifestUseCase,
}

impl UpCommand {
    pub fn new(
        manifest: ManifestUseCase,
    ) -> Self {
        Self{
            manifest,
        }
    }
    pub fn spec(&self) -> Command {
        Command::new("up")
            .about("up command")
    }
    pub fn action(&self) -> Result<(), &str> {
        let manifest = self.manifest.get_manifest()
            .ok_or("[ERROR] manifest is not found.")?;
        let lang = manifest["lang"].as_str()
            .ok_or("[ERROR] lang is a required field. lang is not set.")?;
        // let spec = manifest["spec"].as_vec()
        //     .ok_or("[ERROR] spec is not set. spec is a required field.")?;
        // let arch = manifest["arch"].as_str()
        //     .unwrap_or("plain");
        // let root = manifest["root"].as_str()
        //     .unwrap_or("");

        match lang {
            "rust" => {
                let uc = RustFileUseCase::new(
                    CodeFileRepository::new(manifest),
                );
            }
        }

        Ok(())
    }
}
