use crate::domain::model::manifest::MANIFEST;
use crate::utils::act::codefile;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

pub struct Rust;

impl Rust {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self{})
    }
    fn modblock_pattern(&self) -> anyhow::Result<Regex> {
        let pattern = r"// start auto exported by saba\.[\s\S]*// end auto exported by saba\.";
        Ok(Regex::new(pattern)?)
    }
    fn main_rs(&self, root: String) -> anyhow::Result<String> {
        let main_rs = if PathBuf::from(root.clone() + "lib.rs").exists()
        {
            PathBuf::from(root.clone() + "lib.rs")
        }
        else if PathBuf::from(root.clone() + "main.rs").exists()
        {
            PathBuf::from(root.clone() + "main.rs")
        }
        else {
            let path = PathBuf::from(root.clone() + "main.rs");
            File::create(path.to_str().unwrap())?;
            path
        };
        let main_rs = main_rs.file_name().unwrap().to_str().unwrap_or("");
        Ok(main_rs.to_string())
    }
}


impl codefile::Act for Rust {
    fn gen_location_post(&self) -> anyhow::Result<()> {
        let manifest = MANIFEST.lock().unwrap();
        let root = manifest.root.clone();
        let main_rs = self.main_rs(root.clone())?;
        let main_rs_path = PathBuf::from(root.clone() + main_rs.as_str());
        let mut file = File::open(main_rs_path.clone())?;
        let mut file_contents = String::new();
        let regx = self.modblock_pattern()?;

        // 新しいmodblockを作成
        let mut modblock = "// start auto exported by saba.\n".to_string();
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap_or("");
            let file_stem = path.file_stem().unwrap().to_str().unwrap_or("");

            if file_name == main_rs.as_str() {
                continue;
            }
            else if file_stem != "" {
                modblock = format!("{}mod {};\n", modblock, file_stem);
            }
        }
        modblock = format!("{}// end auto exported by saba.\n", modblock);

        // 今のファイルの内容を読み込み
        file.read_to_string(&mut file_contents)?;

        // ファイル内にパターンが見つかった場合は置換
        if regx.is_match(&file_contents) {
            let replaced_contents = regx.replace_all(&file_contents, modblock);
            let mut new_file = File::create(main_rs_path)?;
            new_file.write_all(replaced_contents.as_bytes())?;
        }
        // ファイル内にパターンが見つからなかった場合はmodblockをファイルの先頭に挿入
        else {
            let temp_file = main_rs_path.with_extension("temp");
            let mut new_file = File::create(&temp_file)?;
            let new_modblock = format!("{}\n\n", modblock);

            // 先頭にmodblockを挿入
            new_file.write_all(new_modblock.as_bytes())?;
            // 元のファイルの内容を挿入
            new_file.write_all(file_contents.as_bytes())?;
            fs::rename(&temp_file, main_rs_path)?;
        }

        Ok(())
    }
    fn gen_upstream_post(&self, wd: PathBuf) -> anyhow::Result<()> {
        Ok(())
    }
}
