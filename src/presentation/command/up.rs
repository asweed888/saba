use clap::Command;
use crate::domain::model::manifest::Lang;
use crate::infrastructure::filesystem::manifest::ManifestRepository;
use crate::usecase::gen_file::rust::gen_file::Rust;
use crate::usecase::gen_file::golang::gen_file::Golang;
use crate::usecase::gen_file::python::gen_file::Python;
use crate::usecase::gen_file::typescript::gen_file::TypeScript;
use crate::usecase::gen_file::bash::gen_file::Bash;
use crate::usecase::gen_file::lua::gen_file::Lua;

pub fn spec() -> Command {
    Command::new("up")
        .about("up command")
}

pub fn action() -> anyhow::Result<()> {
    let repo = ManifestRepository::new()?;

    match repo.manifest.lang {
        Lang::Rust => {
            let rust = Rust::new(&repo)?;
            rust.gen_file()?;
        },
        Lang::Golang => {
            let golang = Golang::new(&repo)?;
            golang.gen_file()?;
        },
        Lang::Python => {
            let python = Python::new(&repo)?;
            python.gen_file()?;
        },
        Lang::TypeScript => {
            let typescript = TypeScript::new(&repo)?;
            typescript.gen_file()?;
        },
        Lang::Bash => {
            let bash = Bash::new(&repo)?;
            bash.gen_file()?;
        },
        Lang::Lua => {
            let lua = Lua::new(&repo)?;
            lua.gen_file()?;
        }
    }
    println!("[Success] generate of saba has been completed.");
    Ok(())
}