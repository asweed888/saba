use yaml_rust::{
    Yaml,
    YamlLoader,
};
use std::fs;

pub fn to_str(value: Option<&str>) -> &str {
    match value {
        Some(v) => {
            v
        }
        None => {
            ""
        }
    }
}

pub fn load<'a>(file_path: &'a str) -> Vec<Yaml> {
    let f = fs::read_to_string(file_path);
    let s = f.unwrap().to_string();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    docs
}
