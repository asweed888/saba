pub struct Lang {}

impl Lang {
    pub fn new() -> Self {
        Self{}
    }
}

pub trait ProgramingLang {
    fn ext() -> String;
}

pub trait Rust {}

pub trait Go {}
