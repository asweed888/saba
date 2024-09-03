use clap::Command;
use anyhow::bail;
use inquire::{Select, Confirm};
use askama::Template;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use crate::utils::templates::manifest::ManifestTemplate;

pub fn spec() -> Command {
    Command::new("new")
        .about("new command")
}

pub fn action() -> anyhow::Result<()> {
    let lang: String;
    let is_ddd: bool;
    let options: Vec<&str> = vec![
        "rust",
        "go",
        "python",
        "typescript",
        "lua",
        "bash",
        "html",
    ];

    let lang_ans = Select::new("Please select a programming language.", options).prompt();

    match lang_ans {
        Ok(choice) => {
            lang = choice.to_string();
            if lang == "bash"
                || lang == "lua"
                || lang == "html"
            {
                // shellの場合はis_dddは常にfalse
                is_ddd = false;
            }
            else {
                // shell以外の言語はdddオプションあり
                let arch_ans = Confirm::new("Do you want to develop applications with ddd (onion architecture)?")
                    .with_default(false)
                    .prompt();
                match arch_ans {
                    Ok(true) => { is_ddd = true }
                    Ok(false) => { is_ddd = false }
                    Err(_) => {
                        bail!("[ERROR] An error occurred while executing the command.")
                    }
                }
            }
        }
        Err(_) => {
            bail!("[ERROR] An error occurred while executing the command.");
        }
    }

    let data = ManifestTemplate{
        lang,
        is_ddd,
    };

    let rendered_tmpl = data.render()?;

    let file_path = Path::new("./saba.yml");
    if !file_path.exists() {
        let mut file = File::create(file_path.to_str().unwrap())?;
        file.write_all(rendered_tmpl.as_bytes())?;
    }

    Ok(())
}
