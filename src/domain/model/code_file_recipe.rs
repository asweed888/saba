pub trait PGLang {
    fn ext() -> String;
}

pub fn new<L>(lang_recipe: L) -> L
where
    L: PGLang,
{
    lang_recipe
}
