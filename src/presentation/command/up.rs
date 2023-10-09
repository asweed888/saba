use crate::domain::model::manifest::Manifest;
use crate::usecase::manifest::ManifestUseCase;
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
        let manifest = self.manifest.get_manifest().unwrap();

        match manifest.lang {
            "rust" => {
                let uc = RustFileUseCase::new(manifest);
                uc.generate_file();
            }
        }

        Ok(())
    }
}
