use getset::{Getters, Setters};

#[derive(Getters, Setters)]
pub struct Root {
    #[getset(get = "pub", set = "pub")]
    path: String,
}

impl Root {
    pub fn new(path: String) -> Self {
        Self{
            path,
        }
    }
}