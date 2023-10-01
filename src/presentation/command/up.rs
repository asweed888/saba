use crate::domain::model::up::Up;
use crate::usecase::up::UpUseCase;


pub struct UpCommand {
    pub usecase: UpUseCase
}

impl UpCommand {
    pub fn new(usecase: UpUseCase) -> Self {
        Self{ usecase }
    }
}
