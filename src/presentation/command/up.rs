use clap::Command;
use anyhow::bail;
use crate::domain::model::manifest::Manifest;
use crate::usecase::rust_usecase::Rust;





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
        }
        "python" => {
        }
        "typescript" => {
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
