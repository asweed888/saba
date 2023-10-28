use crate::domain::manifest::entity::ManifestRepository;
use crate::usecase::manifest::basic::ManifestUseCase;
use crate::presentation::command::up::UpCommand;
use crate::presentation::command::new::NewCommand;

pub struct App {
    pub up_cmd: UpCommand,
    pub new_cmd: NewCommand,
}

pub struct DIContainer {}

impl DIContainer {
    pub fn new() -> Self {
        Self{}
    }
    pub fn new_app(&self) -> App {
        App{
            up_cmd: self.new_up_cmd(),
            new_cmd: self.new_new_cmd(),
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
    fn new_new_cmd(&self) -> NewCommand {
        NewCommand::new()
    }
}