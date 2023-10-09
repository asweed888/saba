use crate::domain::repository::code_file::CodeFileRepository;
use yaml_rust::Yaml;

impl<'a> CodeFileRepository<'a> {
    pub fn new(manifest: &'a Yaml) -> Self {
        Self{ manifest }
    }
}
