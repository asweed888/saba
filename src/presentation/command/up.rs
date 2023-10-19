use clap::Command;
use crate::usecase::manifest::basic::ManifestUseCase;
use crate::usecase::manifest::rust::RustUseCase;

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
        let mut manifest = self.manifest.load().unwrap();

        match manifest.lang.name().as_str() {
            "rust" => {
                manifest.root.set_path(String::from("./src"));
                let uc = RustUseCase::new(manifest);
                uc.gen_file();
            }
            _ => {

            }
        }
        Ok(())
    }
}