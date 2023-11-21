use getset::{Getters, Setters};

#[derive(Getters, Setters, Debug)]
pub struct Root {
    #[getset(get = "pub", set = "pub")]
    path: String,
    #[getset(get = "pub", set = "pub")]
    default: String,
}

impl Root {
    pub fn new(path: String) -> Self {
        Self{
            path,
            default: String::from(""),
        }
    }
    pub fn get_path(&self) -> String {
        match self.path().as_str() {
            "" => { self.default().to_string() }
            _ => { self.path().to_string() }
        }
    }
}