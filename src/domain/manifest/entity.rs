use yaml_rust::Yaml;
use crate::domain::manifest::root::Root;
use crate::domain::manifest::arch::Arch;
use crate::domain::manifest::lang::Lang;

#[derive(Debug)]
pub struct Manifest {
    pub lang: Lang,
    pub arch: Arch,
    pub root: Root,
    pub spec: Vec<Yaml>,
}

pub struct ManifestRepository{}