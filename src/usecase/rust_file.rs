use crate::domain::model::manifest::Manifest;
use crate::usecase::interface::CodeFileGenerator;

pub struct RustFileUseCase<'a> {
    pub repository: Manifest<'a>,
}

impl<'a> RustFileUseCase<'a> {
    pub fn new(repository: Manifest<'a>) -> Self {
        Self{ repository }
    }
    pub fn generate_file(&self) {
        self.gen_file(self.repository)
    }
}

impl<'a> CodeFileGenerator<'a> for RustFileUseCase<'a> {
    fn gen_file(&self, repository: Manifest<'a>) {

        for spec in repository.spec {
            spec["location"].as_str().unwrap();
        }
    }
}
