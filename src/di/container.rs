use crate::domain::manifest::entity::TManifestRepository;
use crate::infrastructure::fs::manifest::ManifestFileSystemRepository;
use crate::usecase::manifest::basic::ManifestUseCase;
use crate::presentation::command::up::UpCommand;


pub struct DIContainer<'a>;

impl<'a> DIContainer<'a> {
    pub fn new() -> Self {
        Self{}
    }
    pub fn new_app(&self) -> (UpCommand<'a, ) {
        let manifest_repository = ManifestFileSystemRepository::new();
        let manifest_usecase = ManifestUseCase::new(manifest_repository);
        let up_cmd = UpCommand::new(&manifest_usecase);
        App{
            up_cmd,
        }
    }
}