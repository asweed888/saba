use clap::Command;
use anyhow::bail;
use crate::domain::model::manifest::Manifest;
use crate::usecase::rust_usecase::Rust;
use crate::usecase::golang_usecase::Golang;
use crate::usecase::python_usecase::Python;
use crate::usecase::typescript_usecase::TypeScript;
use crate::usecase::lua_usecase::Lua;
use crate::usecase::bash_usecase::Bash;
use crate::usecase::html_usecase::Html;

pub fn spec() -> Command {
    Command::new("up")
        .about("up command")
}

pub fn action() -> anyhow::Result<()> {
    let mut manifest = Manifest::new()?;

    match manifest.lang.as_str() {
        "rust" => {
            let rust = Rust::new(&mut manifest)?;
            rust.gen_file()?;
        }
        "go" => {
            let golang = Golang::new(&mut manifest)?;
            golang.gen_file()?;
        }
        "python" => {
            let python = Python::new(&mut manifest)?;
            python.gen_file()?;
        }
        "typescript" => {
            let typescript = TypeScript::new(&mut manifest)?;
            typescript.gen_file()?;
        }
        "lua" => {
            let lua = Lua::new(&mut manifest)?;
            lua.gen_file()?;
        }
        "bash" => {
            let bash = Bash::new(&mut manifest)?;
            bash.gen_file()?;
        }
        "html" => {
            let html = Html::new(&mut manifest)?;
            html.gen_file()?;
        }
        _ => {
            bail!("[ERROR] The language is not supported.")
        }
    }
    println!("[Success] generate of saba has been completed.");
    Ok(())
}
