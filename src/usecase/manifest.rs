use crate::domain::repository::manifest::ManifestRepository;
use yaml_rust::Yaml;


pub struct ManifestUseCase {
    pub repository: ManifestRepository
}

impl ManifestUseCase {
    pub fn new(repository: ManifestRepository) -> Self {
        Self{ repository }
    }
    pub fn get_manifest(&self) -> Option<&Yaml> {
        let raw = self.repository.load_manifest();
        raw.get(0)
    }
}
