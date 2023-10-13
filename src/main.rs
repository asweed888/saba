mod entity {
    pub mod manifest {
        pub mod prelude;
        pub mod root;
    }
    pub mod manifest;
}
mod usecase {
    pub mod rust;
} // Automatically exported by saba.

use clap::{arg, Command};
use crate::di::container::DIContainer;


fn main() {

    let manifest_file_name = "./saba.yml";
    let dic = DIContainer::new();
    let app = dic.new_app();

    let matches = Command::new("saba")
        .about("This is a very simple declarative development framework.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            app.up_cmd.spec()
        )
        .get_matches();

    match matches.subcommand() {
        Some(("up", up_matches)) => {
        }
        _ => unreachable!()
    }
}
