use yaml_rust::Yaml;

pub struct Manifest<'a> {
    pub lang: &'a str,
    pub arch: &'a str,
    pub root: &'a str,
    pub spec: &'a Vec<Yaml>,
}
