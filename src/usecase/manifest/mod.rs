pub struct ManifestUseCase {
    repository: ManifestRepository,
}

impl ManifestUseCase {
    pub fn new(repository: ManifestRepository) -> Self {
        Self{
            repository,
        }
    }
    pub fn load(&self) -> Result<Manifest, &str> {
        self.repository;
    }
}
