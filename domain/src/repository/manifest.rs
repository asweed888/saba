use anyhow::Result;
use crate::model::manifest::entity::Manifest;

pub trait ManifestRepository {
    fn load(&self) -> Result<Manifest>;
}