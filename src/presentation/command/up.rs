use clap::Command;
use crate::usecase::manifest::ManifestUseCase;
use crate::usecase::rust::RustUseCase;

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
                let u = RustUseCase::new(manifest);
                u.gen_file();
            }
        }
        Ok(())
    }
}