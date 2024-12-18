use yaml_rust::Yaml;
use std::path::PathBuf;
use anyhow::bail;

#[derive(Clone)]
pub struct Manifest {
    pub lang: Lang,
    pub arch: Arch,
    pub root: PathBuf,
    pub spec: Vec<Yaml>,
}

#[derive(Clone)]
pub enum Lang {
    Rust,
    Golang,
    Python,
    TypeScript,
    Bash,
    Lua,
}

impl Lang {
    pub fn from_raw(lang: &str) -> anyhow::Result<Self> {
        match lang {
            "rust" => Ok(Lang::Rust),
            "go" => Ok(Lang::Golang),
            "python" => Ok(Lang::Python),
            "typescript" => Ok(Lang::TypeScript),
            "bash" => Ok(Lang::Bash),
            "lua" => Ok(Lang::Lua),
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
            Lang::Lua => "lua",
        }
    }
    pub fn default_root(&self) -> &str {
        match self {
            Lang::Rust => "./src",
            Lang::Golang => ".",
            Lang::Python => ".",
            Lang::TypeScript => ".",
            Lang::Bash => ".",
            Lang::Lua => ".",
        }
    }
    pub fn is_generate_ignore(&self, filename: &str) -> bool {
        match self {
            Lang::Rust => {
                vec!["main", "mod"].iter().any(|s| *s == filename)
            },
            Lang::Golang => {
                vec!["main"].iter().any(|s| *s == filename)
            },
            Lang::Python => {
                vec!["main", "__init__"].iter().any(|s| *s == filename)
            },
            Lang::TypeScript => {
                vec!["main"].iter().any(|s| *s == filename)
            },
            Lang::Bash => false,
            Lang::Lua => {
                vec!["main"].iter().any(|s| *s == filename)
            },
        }
    }
}

#[derive(Clone)]
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
        }
    }
}