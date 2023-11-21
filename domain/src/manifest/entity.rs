use yaml_rust::Yaml;
use crate::manifest::lang::Lang;
use crate::manifest::arch::Arch;
use crate::manifest::root::Root;

#[derive(Debug)]
pub struct Manifest {
    pub lang: Lang,
    pub arch: Arch,
    pub root: Root,
    pub spec: Vec<Yaml>,
}