
pub struct CodeFile<M>
where
    M: ProgrammingLang,
{
    meta: M,
}


impl<M> CodeFile<M>
where
    M: ProgrammingLang,
{
    pub fn new(meta: M) -> Self {
        Self{
            meta,
        }
    }
}


trait ProgrammingLang  {}

struct Rust {}

impl ProgrammingLang for Rust {

}
