pub struct Lang<'a> {
    pub name: &'a str,
    pub ext: &'a str,
}

impl<'a> Lang<'a> {
    pub fn new(name: &'a str) -> Self {
        Self{
            name,
            ext: "",
        }
    }
}