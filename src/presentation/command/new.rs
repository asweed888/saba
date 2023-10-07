use crate::domain::model::new::New;
use crate::usecase::new::NewUseCase;


pub struct NewCommand {
    pub usecase: NewUseCase
}

impl NewCommand {
    pub fn new(usecase: NewUseCase) -> Self {
        Self{ usecase }
    }
}
