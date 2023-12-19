use clap::Command;
use anyhow::{bail, Result};
use domain::repository::manifest::ManifestRepository;
use usecase::manifest::basic::ManifestUseCase;
use usecase::manifest::rust::basic::RustUseCase;
use usecase::manifest::golang::basic::GoLangUseCase;
use usecase::manifest::python::basic::PythonUseCase;
use usecase::manifest::bash::basic::BashUseCase;
use usecase::manifest::typescript::basic::TypeScriptUseCase;
use usecase::manifest::lua::basic::LuaUseCase;


pub struct UpCommand<R>
where
    R: ManifestRepository,
{
    pub manifest: ManifestUseCase<R>,
}

impl<R> UpCommand<R>
where
    R: ManifestRepository,
{
    pub fn new(manifest: ManifestUseCase<R>) -> Self {
        Self{ manifest }
    }
    pub fn spec(&self) -> Command {
        Command::new("up")
            .about("up command")
    }
    pub fn action(&self) -> Result<()> {
        let mut manifest = self.manifest.load().unwrap();

        match manifest.lang.name().as_str() {
            "rust" => {
                manifest.lang.set_ext(String::from("rs"));
                manifest.root.set_default(String::from("./src"));
                let uc = RustUseCase::new(manifest);
                uc.gen_file()?;
            }
            "go" => {
                manifest.lang.set_ext(String::from("go"));
                manifest.root.set_default(String::from("."));
                let uc = GoLangUseCase::new(manifest);
                uc.gen_file()?;
            }
            "python" => {
                manifest.lang.set_ext(String::from("py"));
                manifest.root.set_default(String::from("."));
                let uc = PythonUseCase::new(manifest);
                uc.gen_file()?;
            }
            "typescript" => {
                manifest.lang.set_ext(String::from("ts"));
                manifest.root.set_default(String::from("./src"));
                let uc = TypeScriptUseCase::new(manifest);
                uc.gen_file()?;
            }
            "lua" => {
                manifest.lang.set_ext(String::from("lua"));
                manifest.root.set_default(String::from("."));
                let uc = LuaUseCase::new(manifest);
                uc.gen_file()?;
            }
            "bash" => {
                manifest.lang.set_ext(String::from(""));
                manifest.root.set_default(String::from("."));
                let uc = BashUseCase::new(manifest);
                uc.gen_file()?;
            }
            _ => {
                bail!("[ERROR] The language is not supported.")
            }
        }
        println!("[Success] generate of saba has been completed.");
        Ok(())
    }
}