use yaml_rust::Yaml;
use crate::model::manifest::lang::Lang;
use crate::model::manifest::arch::Arch;
use crate::model::manifest::root::Root;

#[derive(Debug)]
pub struct Manifest {
    pub lang: Lang,
    pub arch: Arch,
    pub root: Root,
    pub spec: Vec<Yaml>,
}