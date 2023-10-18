use crate::domain::manifest::entity::{
    Manifest,
    TManifestRepository,
};


pub struct ManifestUseCase<'a, R>
where
    R: TManifestRepository<'a>,
{
    pub repository: R,
}

impl<'a, R> ManifestUseCase<'a, R> {
    pub fn new(repository: R) -> Self {
        Self{
            repository,
        }
    }
    pub fn load(&self) -> Result<Manifest, &str> {
        self.repository;
    }
}
