use crate::domain::manifest::entity::TManifestRepository;


pub struct ManifestFileSystemRepository;

impl ManifestFileSystemRepository {
    pub fn new() -> Self {
        Self{}
    }
}

impl<'a> TManifestRepository<'a> for ManifestFileSystemRepository {}
