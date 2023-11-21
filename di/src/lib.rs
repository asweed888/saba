use infrastructure::repository::manifest::ManifestRepository;
use usecase::manifest::basic::ManifestUseCase;
use presentation::command::up::UpCommand;
use presentation::command::new::NewCommand;

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
