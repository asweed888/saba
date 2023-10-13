pub struct Root<'a> {
    path: &'a str,
}

impl<'a> Root<'a> {
    pub fn new(path: &'a str) -> Self {
        Self{ path }
    }
    pub fn get_path(&self, default: &'a str) -> String {
        match self.path {
            "" => { String::from(default) }
            _ => { String::from(self.path) }
        }
    }
}