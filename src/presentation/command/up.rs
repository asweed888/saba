use clap::Command;
use crate::usecase::manifest::basic::ManifestUseCase;

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
    pub fn action(&self) -> Result<(), &str> {
        let manifest = self.manifest.load().unwrap();

        match manifest.lang {
            "rust" => {
            }
        }
        Ok(())
    }
}