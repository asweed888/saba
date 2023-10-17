use crate::domain::manifest::entity::Manifest;

pub trait TGenerateFileUseCase<'a> {
    fn location_action(&self, manifest: Manifest<'a>) {
    }
}
