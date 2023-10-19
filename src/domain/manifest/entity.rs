use yaml_rust::Yaml;
use crate::domain::manifest::root::Root;
use crate::domain::manifest::arch::Arch;
use crate::domain::manifest::lang::Lang;

pub struct Manifest<'a> {
    pub lang: Lang,
    pub arch: Arch,
    pub root: Root,
    pub spec: &'a Vec<Yaml>,
}

pub struct ManifestRepository{}