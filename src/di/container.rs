use crate::domain::repository::{
    manifest::ManifestRepository,
    rust_file::RustFileRepository,
    go_file::GoFileRepository,
};
use crate::usecase::{
    manifest::ManifestUseCase,
    rust_file::RustFileUseCase,
    go_file::GoFileUseCase,
};
use crate::presentation::command::{
    new::NewCommand,
    up::UpCommand,
};

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
            up_cmd: self.new_up_cmd()
        }
    }
    fn new_up_cmd(&self) -> UpCommand {
        UpCommand::new(
            self.new_manifest_usecase(),
        )
    }
    fn new_manifest_repository(&self) -> ManifestRepository {
        ManifestRepository::new()
    }
    fn new_manifest_usecase(&self) -> ManifestUseCase {
        ManifestUseCase::new(self.new_manifest_repository())
    }
}
