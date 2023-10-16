pub struct Root<'a> {
    pub path: &'a str,
    pub default: &'a str,
}

impl<'a> Root<'a> {
    pub fn new(path: &'a str) -> Self {
        Self{
            path,
            default: ".",
        }
    }
    pub fn get_path(&self) -> String {
        match self.path {
            "" => { String::from(self.default) }
            _ => { String::from(self.path) }
        }
    }
}