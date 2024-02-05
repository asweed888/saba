mod usecase {
    pub mod generate {
        pub mod codefile {
            pub mod rust {
                pub mod slim;
                pub mod template;
                pub mod utils;
            }
            pub mod golang {
                pub mod r#mod;
                pub mod template;
            }
            pub mod python {
                pub mod r#mod;
                pub mod template;
            }
            pub mod typescript {
                pub mod r#mod;
                pub mod template;
            }
            pub mod lua {
                pub mod r#mod;
            }
            pub mod bash {
                pub mod r#mod;
            }
        }
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
