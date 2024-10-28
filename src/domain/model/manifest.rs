use yaml_rust::Yaml;
use yaml_rust::YamlLoader;
use std::default::Default;
use std::path::PathBuf;
use anyhow::bail;

#[derive(Default, Clone)]
pub struct Manifest {
    pub lang: Lang,
    pub arch: Arch,
    pub root: PathBuf,
    pub spec: Vec<Yaml>,
}

pub enum Lang {
    Rust,
    Golang,
    Python,
    TypeScript,
    Bash,
}

impl Lang {
    pub fn from_raw(lang: &str) -> anyhow::Result<Self> {
        match lang {
            "rust" => Ok(Lang::Rust),
            "go" => Ok(Lang::Golang),
            "python" => Ok(Lang::Python),
            "typescript" => Ok(Lang::TypeScript),
            "bash" => Ok(Lang::Bash),
            _ => bail!("The programming language is not supported."),
        }
    }
    pub fn ext(&self) -> &str {
        match self {
            Lang::Rust => "rs",
            Lang::Golang => "go",
            Lang::Python => "py",
            Lang::TypeScript => "ts",
            Lang::Bash => "",
        }
    }
    pub fn default_root(&self) -> &str {
        match self {
            Lang::Rust => "./src",
            Lang::Golang => ".",
            Lang::Python => ".",
            Lang::TypeScript => ".",
            Lang::Bash => ".",
        }
    }
    pub fn is_generate_ignore(&self, filename: &str) -> bool {
        match self {
            Lang::Rust => {
                vec!["main.rs", "mod.rs"].iter().any(|s| *s == filename)
            },
            Lang::Golang => {
                vec!["main.go"].iter().any(|s| *s == filename)
            },
            Lang::Python => {
                vec!["main.py", "__init__.py"].iter().any(|s| *s == filename)
            },
            Lang::TypeScript => {
                vec!["main.ts"].iter().any(|s| *s == filename)
            },
            Lang::Bash => false,
        }
    }
}

pub enum Arch {
    DDD,
    Plain,
}

impl Arch {
    pub fn from_raw(arch: &str) -> anyhow::Result<Self> {
        match arch {
            "ddd" => Ok(Arch::DDD),
            _ => Ok(Arch::Plain),
        }
    }
    pub fn is_ddd(&self) -> bool {
        match self {
            Arch::DDD => true,
            Arch::Plain => false,
            _ => false,
        }
    }
}