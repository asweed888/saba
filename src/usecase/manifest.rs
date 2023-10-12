use crate::domain::model::manifest::Manifest;
use crate::domain::repository::manifest::ManifestRepository;


pub struct ManifestUseCase {
    pub repository: ManifestRepository
}

impl ManifestUseCase {
    pub fn new(repository: ManifestRepository) -> Self {
        Self{ repository }
    }
    pub fn get_manifest(&self) -> Result<Manifest, &str> {
        self.repository.load_manifest()
    }
}