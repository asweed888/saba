use crate::domain::model::spec::Spec;
use crate::domain::repository::manifest::ManifestRepository;
use std::convert::From;
use yaml_rust::Yaml;


impl ManifestRepository {
    pub fn new(yaml: Vec<Yaml>) -> Self {
        Self{ yaml }
    }
}

impl Vec<Yaml> for Spec {
    fn from()
}
