use crate::infrastructure::repository::manifest::ManifestRepository;
use crate::usecase::manifest::basic::ManifestUseCase;
use crate::presentation::command::up::UpCommand;

pub struct App {
    pub up_cmd: UpCommand,
}

pub struct DIContainer {}

impl DIContainer {
    pub fn new() -> Self {
        Self{}
    }
    pub fn new_app(&self) -> App {
        App{
            up_cmd: self.new_up_cmd(),
        }
    }
    fn new_manifest_repository(&self) -> ManifestRepository {
        ManifestRepository::new()
    }
    fn new_manifest_usecase(&self) -> ManifestUseCase {
        ManifestUseCase::new(self.new_manifest_repository())
    }
    fn new_up_cmd(&self) -> UpCommand {
        UpCommand::new(self.new_manifest_usecase())
    }
}