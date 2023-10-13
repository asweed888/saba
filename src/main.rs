mod utils {
    pub mod template {
        pub mod prelude;
        pub mod rust;
    }
}
mod domain {
    pub mod model {
        pub mod manifest;
    }
    pub mod repository {
        pub mod manifest;
    }
}
mod infrastructure {
    pub mod repository {
        pub mod manifest;
    }
}
mod usecase {
    pub mod interface {
        pub mod code_file_generator;
    }
    pub mod manifest;
    pub mod rust_file;
    pub mod go_file;
}
mod presentation {
    pub mod command {
        pub mod new;
        pub mod up;
    }
}
mod di {
    pub mod container;
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
