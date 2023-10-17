use crate::domain::manifest::entity::Manifest;
use crate::usecase::generate_file::GenerateFileUseCase;

pub struct RustUseCase<'a> {
    manifest: Manifest<'a>,
}

impl<'a> RustUseCase<'a> {
    pub fn new(manifest: Manifest<'a>) -> Self {
        Self{
            manifest,
        }
    }
    pub fn gen_file(&self){
        self.location_action(self.manifest);
    }
}

impl<'a> GenerateFileUseCase<'a> for RustUseCase<'a> {}