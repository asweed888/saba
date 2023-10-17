use crate::domain::manifest::entity::Manifest;

pub trait GenerateFileUseCase<'a> {
    fn location_action(&self, manifest: Manifest<'a>) {
    }
}