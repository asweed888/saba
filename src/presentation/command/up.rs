use clap::Command;
use crate::domain::manifest::entity::TManifestRepository;
use crate::usecase::manifest::basic::ManifestUseCase;

pub struct UpCommand<'a, R>
where
    R: TManifestRepository<'a>,
{
    pub manifest: ManifestUseCase<'a, R>,
}

impl<'a, T> UpCommand<'a, T> {
    pub fn new(manifest: ManifestUseCase<'a, T>) -> Self {
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