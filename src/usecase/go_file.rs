use crate::domain::repository::code_file::CodeFileRepository;


pub struct GoFileUseCase<'a> {
    pub repository: CodeFileRepository<'a>,
}

impl<'a> GoFileUseCase<'a> {
    pub fn new(repository: CodeFileRepository) -> Self {
        Self{ repository }
    }
    pub fn gen_file(&self) {
        let manifest = self.repository.manifest;

    }
}
