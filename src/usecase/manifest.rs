use crate::domain::manifest::entity::ManifestRepository;

pub struct ManifestUseCase {}

impl  ManifestUseCase {
    pub fn new() -> Self {
        Self{}
    }
}

impl<'a> ManifestRepository<'a> for ManifestUseCase {}