use crate::domain::model::manifest::Manifest;
use crate::domain::repository::manifest::ManifestRepository;
use yaml_rust::Yaml;


impl ManifestRepository {
    pub fn new(yaml: Vec<Yaml>) -> Self {
        Self{ yaml }
    }
}
