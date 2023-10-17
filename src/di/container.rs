use crate::usecase::manifest::ManifestUseCase;
use crate::presentation::command::up::UpCommand;

pub struct App<'a> {
    pub up_cmd: UpCommand<'a>,
}

pub struct DIContainer<'a> {};

impl DIContainer {
    pub fn new() -> Self {
        Self{}
    }
    pub fn new_app(&self) -> App {
        App{
            up_cmd: self.new_up_cmd(),
        }
    }
    fn new_up_cmd(&self) -> UpCommand {

    }
}