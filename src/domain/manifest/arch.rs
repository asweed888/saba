use getset::{Getters, Setters};

#[derive(Getters, Setters)]
pub struct Arch {
    #[getset(get = "pub", set = "pub")]
    kind: String,
}

impl Arch {
    pub fn new(kind: String) -> Self {
        Self{ kind }
    }
    pub fn is_ddd(&self) -> bool {
        self.kind == "ddd"
    }
}