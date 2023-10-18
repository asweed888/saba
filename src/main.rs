mod domain {
    pub mod manifest {
        pub mod entity;
        pub mod lang;
        pub mod arch;
        pub mod root;
    }
}
mod infrastructure {
    pub mod repository {
        pub mod manifest;
    }
}
mod usecase {
    pub mod manifest {
        pub mod basic;
        pub mod interface;
        pub mod rust;
    }
}
mod presentation {
    pub mod command {
        pub mod up;
        pub mod new;
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
