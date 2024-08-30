pub mod utils {
    pub mod templates {
        pub mod rust;
    }
    pub mod act {
        pub mod gen {
            pub mod codefile;
        }
        pub mod write {
            pub mod codefile;
        }
        pub mod codefile;
    }
}
pub mod domain {
    pub mod model {
        pub mod manifest;
    }
}
pub mod usecase {
    pub mod rust_usecase;
}
pub mod presentation {
    pub mod command {
        pub mod up;
        pub mod new;
    }
}
// Automatically exported by saba.


// use clap::Command;
// use anyhow::Result;
// use crate::di::container::DIContainer;
//
// fn main() -> Result<()> {
//     let dic = DIContainer::new();
//     let app = dic.new_app();
//     let version = env!("CARGO_PKG_VERSION");
//
//     let matches = Command::new("saba")
//         .about("This is a very simple declarative development framework.")
//         .subcommand_required(true)
//         .arg_required_else_help(true)
//         .subcommand(
//             app.up_cmd.spec()
//         )
//         .subcommand(
//             app.new_cmd.spec()
//         )
//         .version(version)
//         .get_matches();
//
//     match matches.subcommand() {
//         Some(("up", _)) => {
//             app.up_cmd.action()
//         }
//         Some(("new", _)) => {
//             app.new_cmd.action()
//         }
//         _ => unreachable!()
//     }
// }

use domain::model::manifest::MANIFEST;

// テスト用
fn main() -> anyhow::Result<()> {
    let mut manifest = MANIFEST.lock().unwrap();



    // manifest.set_ext("rs");
    //
    // println!("拡張子: {}", manifest.ext);

    manifest.set_root("./lang_src");
    println!("root: {}", manifest.root);
    Ok(())
}
