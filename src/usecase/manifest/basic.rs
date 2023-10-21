use crate::domain::manifest::entity::{
    Manifest,
    ManifestRepository,
};


pub struct ManifestUseCase {
    pub repository: ManifestRepository,
}

impl ManifestUseCase {
    pub fn new(repository: ManifestRepository) -> Self {
        Self{
            repository,
        }
    }
    pub fn load(&self) -> Result<Manifest, &str> {
        self.repository.load()
    }
}