// start auto exported by saba.
mod domain;
mod presentation;
mod utils;
mod usecase;
// end auto exported by saba.

use clap::Command;
use crate::presentation::command;

fn main() -> anyhow::Result<()> {

    let version = env!("CARGO_PKG_VERSION");

    let matches = Command::new("saba")
        .about("This is a very simple declarative development framework.")
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
        Some(("new", _)) => {
            command::new::action()
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
