use crate::usecase::manifest::ManifestUseCase;
use crate::usecase::rust_file::RustFileUseCase;
use crate::usecase::go_file::GoFileUseCase;
use clap::{ Command };

pub struct UpCommand {
    pub manifest: ManifestUseCase,
    pub rust_file: RustFileUseCase,
    pub go_file: GoFileUseCase,
}

impl UpCommand {
    pub fn new(
        manifest: ManifestUseCase,
        rust_file: RustFileUseCase,
        go_file: GoFileUseCase,
    ) -> Self {
        Self{
            manifest,
            rust_file,
            go_file,
        }
    }
    pub fn config(&self) -> Command {
        Command::new("up")
            .about("up command")
    }
    pub fn action(&self) -> Result<(), &str> {
        let manifest = self.manifest.get_manifest()
            .ok_or("[ERROR] manifest is not found.")?;
        let lang = manifest["lang"].as_str()
            .ok_or("[ERROR] lang is a required field. lang is not set.")?;


        Ok(())
    }
}
