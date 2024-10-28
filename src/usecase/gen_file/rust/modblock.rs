use regex::Regex;
use std::path::PathBuf;
use std::fs::{self, File};
use anyhow::anyhow;
use crate::infrastructure::filesystem::manifest::ManifestRepository;

#[derive(Clone)]
pub struct ModBlock<'a> {
    header: String,
    body: String,
    footer: String,
    pattern: Regex,
    repo: &'a ManifestRepository,
    workdir: PathBuf,
}

impl<'a> ModBlock<'a> {
    pub fn new(workdir: PathBuf, repo: &'a ManifestRepository) -> anyhow::Result<Self> {
        let header = "// start auto exported by saba.\n".to_string();
        let footer = "// end auto exported by saba.".to_string();
        let pattern = r"// start auto exported by saba\.[\s\S]*// end auto exported by saba\.";

        Ok(Self{
            header,
            body: String::new(),
            footer,
            pattern: Regex::new(pattern)?,
            repo,
            workdir,
        })
    }
    fn is_root(&self) -> bool {
        self.repo.manifest.root == self.workdir
    }
    fn exists_main_rs(&self) -> bool {
        PathBuf::from(self.workdir + "/" + "main.rs").exists()
    }
    fn exists_lib_rs(&self) -> bool {
        PathBuf::from(self.workdir + "/" + "lib.rs").exists()
    }
    fn exists_mod_rs(&self) -> bool {
        PathBuf::from(self.workdir + "/" + "mod.rs").exists()
    }
    fn get_target_file_path(&self) -> anyhow::Result<PathBuf> {
        if self.is_root() {
            if self.exists_lib_rs() {
                Ok(PathBuf::from(self.workdir + "/" + "lib.rs"))
            }
            else if self.exists_main_rs() {
                Ok(PathBuf::from(self.workdir + "/" + "main.rs"))
            }
            else {
                let path = PathBuf::from(self.workdir + "/" + "main.rs");
                File::create(path)?;
                Ok(path)
            }
        }
        else {
            if self.exists_mod_rs() {
                Ok(PathBuf::from(self.workdir + "/" + "mod.rs"))
            }
            else {
                let path = PathBuf::from(self.workdir + "/" + "mod.rs");
                File::create(path)?;
                Ok(path)
            }
        }
    }
    fn modblock(&self) -> String {
        format!(
            "{}{}{}",
            self.header,
            self.body,
            self.footer,
        )
    }
    pub fn update_body(&self, dirname: &str, visivity: &str) -> anyhow::Result<()> {
        let visivity = if self.is_root() && visivity == "" {
            Visibility::Private
        }
        else {
            Visibility::from_raw(visivity)?
        };

        self.body = format!(
            "{}{} mod {};\n",
            self.body,
            visivity.to_code(),
            dirname,
        );
        Ok(())
    }
    pub fn update_workdir(&self, workdir: PathBuf) -> anyhow::Result<()> {
        self.workdir = workdir;
        Ok(())
    }
    pub fn gen(&self) -> anyhow::Result<()> {
        let file_path = self.get_target_file_path()?;
        let mut file = File::open(file_path.clone())?;
        let mut file_contents = String::new();
        let regx = self.pattern;

        // 今のファイルの内容を読み込み
        file.read_to_string(&mut file_contents);

        // ファイル内にパターンが見つかった場合は置換
        if regx.is_match(&file_contents) {
            let replaced_contents = regx.replace_all(&file_contents, self.modblock());
            let mut new_file = File::create(file_path)?;
            new_file.write_all(replaced_contents.as_bytes())?;
        }
        // ファイル内にパターンが見つからなかった場合はmodblockをファイルの先頭に追加
        else {
            let temp_file = file_path.with_extension("temp");
            let mut new_file = File::create(&temp_file)?;
            let new_modblock = format!("{}\n\n", self.modblock());

            // 先頭にmodblockを挿入
            new_file.write_all(new_modblock.as_bytes())?;
            // 元のファイルの内容を挿入
            new_file.write_all(file_contents.as_bytes())?;
            fs::rename(&temp_file, file_path)?;
        }

        Ok(())
    }
}

pub enum Visibility {
    Public,
    Crate,
    Super,
    Private,
}

impl Visibility {
    pub fn from_raw(raw: &str) -> anyhow::Result<Self> {
        match raw {
            "public" => Ok(Visibility::Public),
            "crate" => Ok(Visibility::Crate),
            "super" => Ok(Visibility::Super),
            "private" => Ok(Visibility::Private),
            _ => Ok(Visibility::Public),
        }
    }
    pub fn to_code(&self) -> &str {
        match self {
            Visibility::Public => "pub",
            Visibility::Crate => "pub(crate)",
            Visibility::Super => "pub(super)",
            Visibility::Private => "",
        }
    }
}


