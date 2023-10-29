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
        pub mod rust {
            pub mod basic;
            pub mod template;
            pub mod utils;
        }
        pub mod basic;
        pub mod interface;
        pub mod utils;
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

use clap::Command;
use anyhow::Result;
use crate::di::container::DIContainer;


fn main() -> Result<()> {
    let dic = DIContainer::new();
    let app = dic.new_app();
    let version = env!("CARGO_PKG_VERSION");

    let matches = Command::new("saba")
        .about("This is a very simple declarative development framework.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            app.up_cmd.spec()
        )
        .subcommand(
            app.new_cmd.spec()
        )
        .version(version)
        .get_matches();

    match matches.subcommand() {
        Some(("up", _)) => {
            app.up_cmd.action()
        }
        Some(("new", _)) => {
            app.new_cmd.action()
        }
        _ => unreachable!()
    }
}
