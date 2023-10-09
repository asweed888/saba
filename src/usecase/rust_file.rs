use crate::domain::model::manifest::Manifest;
use crate::usecase::interface::CodeFileGenerator;

pub struct RustFileUseCase<'a> {
    pub repository: Manifest<'a>,
}

impl<'a> RustFileUseCase<'a> {
    pub fn new(repository: Manifest) -> Self {
        Self{ repository }
    }
    pub fn generate_file(&self) {
        self.gen_file()
    }
}

impl<'a> CodeFileGenerator for RustFileUseCase<'a> {
    fn gen_file(&self) {

    }
}
