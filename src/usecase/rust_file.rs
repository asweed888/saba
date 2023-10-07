use crate::domain::model::rust_file::RustFile;
use crate::domain::repository::rust_file::RustFileRepository;


pub struct RustFileUseCase {
    pub repository: RustFileRepository
}

impl RustFileUseCase {
    pub fn new(repository: RustFileRepository) -> Self {
        Self{ repository }
    }
}
