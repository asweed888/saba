use crate::domain::model::manifest::Manifest;

pub trait CodeFileGenerator<'a> {
    fn gen_file(&self, repository: Manifest<'a>) {}
    fn get_root_path(
        &self,
        repository: Manifest<'a>,
        default_root: &str,
    ) -> &str {
        match repository.root {
            "" => { default_root }
            _ => { repository.root }
        }
    }
}
