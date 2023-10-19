use getset::{CopyGetters, Setters};


#[derive(CopyGetters, Setters, Default)]
pub struct Root<T>
where
    T: Copy + Clone + Default,
{
    #[getset(get = "pub", set = "pub")]
    path: T,
}

impl<T> Root<T> {
    pub fn new(path: String) -> Self {
        Self{
            path,
        }
    }
}