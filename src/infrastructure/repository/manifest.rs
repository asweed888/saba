use crate::domain::manifest::entity::Manifest;
use crate::domain::manifest::entity::TManifestRepository;

pub struct ManifestRepository {}

impl ManifestRepository {
    pub fn new() -> Self {
        Self{}
    }
    pub fn load(&self) -> Result<Manifest, &str> {
        self.load_manifest()
    }
}

impl<'a> TManifestRepository<'a> for ManifestRepository {}
