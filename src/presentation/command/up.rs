use clap::Command;
use anyhow::bail;
use crate::domain::model::manifest::Manifest;
use crate::usecase::rust_usecase::Rust;
use crate::usecase::golang_usecase::Golang;
use crate::usecase::python_usecase::Python;
use crate::usecase::typescript_usecase::TypeScript;

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
        }
        "bash" => {
        }
        "html" => {
        }
        _ => {
            bail!("[ERROR] The language is not supported.")
        }
    }
    println!("[Success] generate of saba has been completed.");
    Ok(())
}
