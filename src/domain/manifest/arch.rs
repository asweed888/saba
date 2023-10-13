pub struct Arch<'a> {
    pub kind: &'a str,
}

impl<'a> Arch<'a> {
    pub fn new(kind: &'a str) -> Self {
        Self{ kind }
    }
    pub fn is_ddd(&self) -> bool {
        self.kind == "ddd"
    }
}