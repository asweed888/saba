use sabacan::manifest::domain::repository::ManifestRepository;
use sabacan::manifest::infra::repository::ManifestRepositoryImpl;
use sabacan::manifest::usecase::load::manifest::ManifestLoadUsecase;
use presentation::command::up::UpCommand;
use presentation::command::new::NewCommand;

pub struct App<R>
where
    R: ManifestRepository,
{
    pub up_cmd: UpCommand<R>,
    pub new_cmd: NewCommand,
}

impl<R> App<R>
where
    R: ManifestRepository,
{
    pub fn new(
        up_cmd: UpCommand<R>,
        new_cmd: NewCommand
    ) -> Self {
        Self{
            up_cmd,
            new_cmd,
        }
    }
}

pub struct DIContainer;

impl DIContainer {
    pub fn new() -> Self {
        Self{}
    }
    pub fn new_app(&self) -> App<ManifestRepositoryImpl> {
        App{
            up_cmd: self.new_up_cmd(),
            new_cmd: self.new_new_cmd(),
        }
    }
    fn new_manifest_repository(&self) -> ManifestRepositoryImpl {
        ManifestRepositoryImpl::new()
    }
    fn new_manifest_load_usecase(&self) -> ManifestLoadUsecase<ManifestRepositoryImpl> {
        ManifestLoadUsecase::new(self.new_manifest_repository())
    }
    fn new_up_cmd(&self) -> UpCommand<ManifestRepositoryImpl> {
        UpCommand::new(self.new_manifest_load_usecase())
    }
    fn new_new_cmd(&self) -> NewCommand {
        NewCommand::new()
    }
}
