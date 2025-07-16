// start auto exported by saba.
mod project_management;
mod code_generation;
mod claude_code_integration;
mod shared;
mod cli;
// end auto exported by saba.

use clap::Command;
use crate::cli::command;

fn main() -> anyhow::Result<()> {

    let version = env!("CARGO_PKG_VERSION");

    let matches = Command::new("saba")
        .about("Saba v2.0 - A declarative development framework for multi-language project generation")
        .long_about(
            "Saba is a declarative development framework that generates project structures \
            from YAML specifications. It supports Rust, Go, Python, TypeScript, and JavaScript \
            with intelligent project structure generation, workspace management, and \
            language-specific configurations."
        )
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            command::up::spec()
        )
        .subcommand(
            command::new::spec()
        )
        .version(version)
        .get_matches();

    match matches.subcommand() {
        Some(("up", _)) => {
            command::up::action()
        }
        Some(("new", sub_matches)) => {
            command::new::action(sub_matches)
        }
        _ => unreachable!()
    }
}

// use domain::model::manifest::MANIFEST;
//
// // テスト用
// fn main() -> anyhow::Result<()> {
//     let mut manifest = MANIFEST.lock().unwrap();
//
//
//
//     // manifest.set_ext("rs");
//     //
//     // println!("拡張子: {}", manifest.ext);
//
//     manifest.set_root("./lang_src");
//     println!("root: {}", manifest.root);
//     Ok(())
// }
