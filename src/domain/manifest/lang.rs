use getset::{Getters, Setters};


#[derive(Getters, Setters, Debug)]
pub struct Lang {
    #[getset(get = "pub", set = "pub")]
    name: String,

    #[getset(get = "pub", set = "pub")]
    ext: String,
}

impl Lang {
    pub fn new(name: String) -> Self {
        Self{
            name,
            ext: String::from(""),
        }
    }
}