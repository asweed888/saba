use crate::domain::model::go_file::GoFile;
use crate::domain::repository::go_file::GoFileRepository;


pub struct GoFileUseCase {
    pub repository: GoFileRepository
}

impl GoFileUseCase {
    pub fn new(repository: GoFileRepository) -> Self {
        Self{ repository }
    }
}
