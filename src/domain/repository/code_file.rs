use yaml_rust::Yaml;

pub struct CodeFileRepository<'a> {
    pub manifest: &'a Yaml,
}
