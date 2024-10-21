use crate::domain::model::manifest::Manifest;
use crate::utils::act::codefile::Act as CodefileAct;
use crate::utils::rust as rs_utils;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use askama::Template;
use crate::utils::templates::rust::{
    DomainModelTmpl,
    DomainRepositoryTmpl,
    InfraTmpl,
    UseCaseTmpl,
    // PresentationTmpl,
    DefaultTmpl,
};

pub struct Rust<'a> {
    manifest: &'a Manifest,
}

impl<'a> Rust<'a> {
    pub fn new(manifest: &'a mut Manifest) -> anyhow::Result<Self> {
        manifest.set_root("./src");
        manifest.set_ext("rs");

        let main_rs = rs_utils::main_rs(manifest.root.clone()).unwrap();
        manifest.set_main_file(main_rs.as_str());
        manifest.set_mod_file("mod.rs");

        Ok(Self{ manifest })
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.gen_location(&self.manifest)?;
        Ok(())
    }
    fn modblock_start(&self) -> String {
        "// start auto exported by saba.\n".to_string()
    }
    fn modblock_end(&self) -> String {
        "// end auto exported by saba.".to_string()
    }
    fn modblock_pattern(&self) -> anyhow::Result<Regex> {
        let pattern = r"// start auto exported by saba\.[\s\S]*// end auto exported by saba\.";
        Ok(Regex::new(pattern)?)
    }
    fn write_modblock(&self, file_path: PathBuf, modblock: String) -> anyhow::Result<()> {
        let mut file = File::open(file_path.clone())?;
        let mut file_contents = String::new();
        let regx = self.modblock_pattern()?;

        // 今のファイルの内容を読み込み
        file.read_to_string(&mut file_contents)?;

        // ファイル内にパターンが見つかった場合は置換
        if regx.is_match(&file_contents) {
            let replaced_contents = regx.replace_all(&file_contents, modblock);
            let mut new_file = File::create(file_path)?;
            new_file.write_all(replaced_contents.as_bytes())?;
        }
        // ファイル内にパターンが見つからなかった場合はmodblockをファイルの先頭に挿入
        else {
            let temp_file = file_path.with_extension("temp");
            let mut new_file = File::create(&temp_file)?;
            let new_modblock = format!("{}\n\n", modblock);

            // 先頭にmodblockを挿入
            new_file.write_all(new_modblock.as_bytes())?;
            // 元のファイルの内容を挿入
            new_file.write_all(file_contents.as_bytes())?;
            fs::rename(&temp_file, file_path)?;
        }
        Ok(())
    }
}


impl<'a> CodefileAct<'a> for Rust<'a> {
    fn gen_codefile_main(&self, wd: PathBuf, manifest: &'a Manifest) -> anyhow::Result<()> {
        let path = wd.to_str().unwrap();

        let is_ddd = manifest.is_ddd();
        let (fname, pkgname) = self.workdir_info(wd.clone(), &manifest);
        let (fname, pkgname) = { (fname.unwrap(), pkgname.unwrap()) };
        let (fname, pkgname) = { (fname.as_str(), pkgname.as_str()) };

        if is_ddd {
            if path.contains("/domain/model/") {
                let data = DomainModelTmpl{fname, pkgname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(wd.to_str().unwrap())?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/domain/repository/") {
                let data = DomainRepositoryTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(wd.to_str().unwrap())?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/infrastructure/") {
                let data = InfraTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(wd.to_str().unwrap())?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/usecase/") {
                let data = UseCaseTmpl{fname, pkgname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(wd.to_str().unwrap())?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            // else if path.contains("/presentation/") {
            //     let data = PresentationTmpl{fname, pkgname};
            //     let rendered_tmpl = data.render()?;
            //     let mut file = File::create(wd.to_str().unwrap())?;
            //     file.write_all(rendered_tmpl.as_bytes())?;
            // }
            else {
                let data = DefaultTmpl{fname, pkgname, wd: wd.to_str().unwrap()};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(wd.to_str().unwrap())?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
        }
        else {
            let data = DefaultTmpl{fname, pkgname, wd: wd.to_str().unwrap()};
            let rendered_tmpl = data.render()?;
            let mut file = File::create(wd.to_str().unwrap())?;
            file.write_all(rendered_tmpl.as_bytes())?;
        }

        Ok(())
    }
    fn gen_location_post(&self, manifest: &'a Manifest) -> anyhow::Result<()> {
        let root = manifest.root.clone();
        let main_rs = manifest.main_file.clone();
        let main_rs_path = PathBuf::from(root.clone() + "/" + main_rs.as_str());

        // 新しいmodblockを作成
        let mut modblock = self.modblock_start();
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path
                .file_name()
                .unwrap()
                .to_str()
                .expect("[ERROR] Failed to get file name.");
            let file_stem = path
                .file_stem()
                .unwrap()
                .to_str()
                .expect("[ERROR] Failed to obtain file stem.");

            if file_name == main_rs.as_str() {
                continue;
            }
            else if file_stem != "" && main_rs.as_str() == "main.rs" {
                modblock = format!("{}mod {};\n", modblock, file_stem);
            }
            else if file_stem != "" && main_rs.as_str() == "lib.rs" {
                modblock = format!("{}pub mod {};\n", modblock, file_stem);
            }
        }
        modblock = format!("{}{}", modblock, self.modblock_end());

        self.write_modblock(main_rs_path, modblock)?;

        Ok(())
    }
    fn gen_upstream_post(&self, wd: PathBuf) -> anyhow::Result<()> {
        let workdir = wd.to_str().unwrap().to_string();
        if workdir.as_str() == "./src" { return Ok(()) }

        let mod_rs = rs_utils::mod_rs(wd.clone())?;
        let mod_rs_path = PathBuf::from(workdir.clone() + "/" + mod_rs.as_str());

        let mut modblock = self.modblock_start();
        for entry in fs::read_dir(workdir)? {
            let entry = entry?;
            let path = entry.path();

            let file_name = path
                .file_name()
                .unwrap()
                .to_str()
                .expect("[ERROR] Failed to get file name.");

            let file_stem = path
                .file_stem()
                .unwrap()
                .to_str()
                .expect("[ERROR] Failed to obtain file stem.");

            if file_name == mod_rs.as_str() {
                continue;
            }
            else {
                modblock = format!("{}pub mod {};\n", modblock, file_stem);
            }
        }
        modblock = format!("{}{}", modblock, self.modblock_end());

        self.write_modblock(mod_rs_path, modblock)?;

        Ok(())
    }
    fn gen_codefile_post(&self, wd: PathBuf) -> anyhow::Result<()> {
        self.gen_upstream_post(wd.clone())
    }
}
