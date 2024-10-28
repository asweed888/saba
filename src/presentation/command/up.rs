use clap::Command;
use anyhow::bail;
use crate::domain::model::manifest::Lang;
use crate::infrastructure::filesystem::manifest::ManifestRepository;
use crate::usecase::gen_file::rust::gen_file::Rust;

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
        _ => {
            bail!("[ERROR] The language is not supported.")
        }
    }
    println!("[Success] generate of saba has been completed.");
    Ok(())
}