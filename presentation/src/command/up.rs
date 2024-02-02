use clap::Command;
use anyhow::bail;
use sabacan::manifest::domain::repository::ManifestRepository;
use sabacan::manifest::usecase::load::manifest::LoadManifestUseCaseImpl;
use usecase::generate::codefile::rust::r#mod::GenerateRustFileUseCaseImpl;
use usecase::generate::codefile::rust;
use usecase::generate::codefile::golang::r#mod::GenerateGoLangFileUseCaseImpl;
use usecase::generate::codefile::python::r#mod::GeneratePythonFileUseCaseImpl;
use usecase::generate::codefile::bash::r#mod::GenerateBashFileUseCaseImpl;
use usecase::generate::codefile::typescript::r#mod::GenerateTypeScriptFileUseCaseImpl;
use usecase::generate::codefile::lua::r#mod::GenerateLuaFileUseCaseImpl;


pub struct UpCommand<R>
where
    R: ManifestRepository,
{
    pub manifest: LoadManifestUseCaseImpl<R>,
}

impl<R> UpCommand<R>
where
    R: ManifestRepository,
{
    pub fn new(manifest: LoadManifestUseCaseImpl<R>) -> Self {
        Self{ manifest }
    }
    pub fn spec(&self) -> Command {
        Command::new("up")
            .about("up command")
    }
    pub fn action(&self) -> anyhow::Result<()> {
        let mut manifest = self.manifest.load().unwrap();

        match manifest.lang.name().as_str() {
            "rust" => {
                manifest.lang.set_ext(String::from("rs"));
                manifest.root.set_default(String::from("./src"));
                let uc = rust::slim::GenerateRustFileUseCaseImpl::new(manifest);
                uc.gen_file()?;
            }
            "go" => {
                manifest.lang.set_ext(String::from("go"));
                manifest.root.set_default(String::from("."));
                let uc = GenerateGoLangFileUseCaseImpl::new(manifest);
                uc.gen_file()?;
            }
            "python" => {
                manifest.lang.set_ext(String::from("py"));
                manifest.root.set_default(String::from("."));
                let uc = GeneratePythonFileUseCaseImpl::new(manifest);
                uc.gen_file()?;
            }
            "typescript" => {
                manifest.lang.set_ext(String::from("ts"));
                manifest.root.set_default(String::from("./src"));
                let uc = GenerateTypeScriptFileUseCaseImpl::new(manifest);
                uc.gen_file()?;
            }
            "lua" => {
                manifest.lang.set_ext(String::from("lua"));
                manifest.root.set_default(String::from("."));
                let uc = GenerateLuaFileUseCaseImpl::new(manifest);
                uc.gen_file()?;
            }
            "bash" => {
                manifest.lang.set_ext(String::from(""));
                manifest.root.set_default(String::from("."));
                let uc = GenerateBashFileUseCaseImpl::new(manifest);
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