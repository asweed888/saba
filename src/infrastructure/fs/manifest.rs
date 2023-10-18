use crate::domain::manifest::entity::Manifest;
use crate::domain::manifest::entity::TManifestRepository;


pub struct ManifestFileSystemRepository;

impl ManifestFileSystemRepository {
    pub fn new() -> Self {
        Self{}
    }
    pub fn load(&self) -> Result<Manifest, &str> {
        self.load()
    }
}

impl<'a> TManifestRepository<'a> for ManifestFileSystemRepository {}
