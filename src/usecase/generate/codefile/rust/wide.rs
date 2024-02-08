use askama::Template;
use getset::{Getters, Setters};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use regex::Regex;
use yaml_rust::Yaml;
use sabacan::manifest::domain::model::entity::Manifest;
use sabacan::manifest::usecase::generate::codefile::{CodefileGenerator, PATH_LIST};
use sabacan::rust::modblock::ModblockHandler;
use sabacan::rust::main_rs;
use crate::usecase::generate::codefile::rust::template::{
    DomainModelTmpl,
    DomainRepositoryTmpl,
    InfraTmpl,
    UseCaseTmpl,
    PresentationTmpl,
    DiTmpl,
    DefaultTmpl,
};

#[derive(Getters, Setters)]
pub struct GenerateRustFileUseCaseImpl {
    manifest: Manifest,
    #[getset(get = "pub", set = "pub")]
    main_rs_path: String,
}

impl<'a> GenerateRustFileUseCaseImpl {
    pub fn new(manifest: Manifest) -> Self {
        Self{
            manifest,
            main_rs_path: String::from(""),
        }
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.location_action(&self.manifest)?;
        self.save_modblock(&self.manifest)?;

        Ok(())
    }
}

impl<'a> ModblockHandler<'a> for GenerateRustFileUseCaseImpl {
    fn save_modblock(&self, manifest: &'a Manifest) -> anyhow::Result<()> {
        // メインとなるファイルのパスの取得とファイルの生成
        let main_rs_path = main_rs::path(&PathBuf::from(manifest.root.get_path()))?;
        main_rs::gen(&main_rs_path)?;

        // ファイルに書き込むmodのブロックを作成
        let mod_block = self.modblock(&manifest, &main_rs_path)?;

        // メインとなるファイルを開き現在の内容を読み込む
        let mut file = File::open(main_rs_path.clone())?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        // mod_blockのパターン
        let re = Regex::new(self.modblock_pattern(&main_rs_path))?;

        if re.is_match(&file_contents) {
            // ファイル内にパターンが見つかった場合は置換
            let replaced_contents = re.replace_all(&file_contents, mod_block.as_str());
            let mut new_file = File::create(main_rs_path)?;
            new_file.write_all(replaced_contents.as_bytes())?;
        }
        else {
            // ファイル内にパターンが見つからなかった場合はmod_blockをファイルの先頭に挿入
            let temp_file = main_rs_path.with_extension("temp");
            let mut new_file = File::create(&temp_file)?;
            let new_mod_block = mod_block.clone() + "\n\n\n";

            // 先頭にmod_blockを挿入
            new_file.write_all(new_mod_block.as_bytes())?;
            // 元のファイルの内容をコピー
            new_file.write_all(file_contents.as_bytes())?;
            fs::rename(&temp_file, main_rs_path)?;
        }

        Ok(())
    }
    fn modblock(&mut self, _manifest: &'a Manifest, _path: &'a PathBuf) -> anyhow::Result<String> {
        let mut mod_block = String::new();
        let vec_default: &Vec<Yaml> = &vec![];

        for (idx, spec) in self.manifest.spec.iter().enumerate() {
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = spec["codefile"].as_vec().unwrap_or(vec_default);

            let main_rs_path = main_rs::path(&PathBuf::from(location))?;
            let main_rs_path_str = main_rs_path.to_str().unwrap().to_string();
            self.set_main_rs_path(main_rs_path_str);

            let main_rs_path = main_rs::path(&PathBuf::from(location))?;
            let path = main_rs_path.to_str().unwrap().to_string();
            let mut tabs = String::new();

            // lib.rsの場合は先頭にpubをつける
            if path.contains("lib.rs") {
                mod_block += "pub ";
            }
            // mod_block += "mod ";
            // mod_block += location;
            // mod_block += " {\n";

            if !upstream.is_empty() {
                self.upstream_modblock(
                    upstream,
                    &mut mod_block,
                    &tabs,
                )?;
                tabs = String::new();
            }

            if !codefile.is_empty() {
                self.codefile_modblock(
                    codefile,
                    &mut mod_block,
                    &tabs,
                )?;
            }

            mod_block.push_str("} // Automatically exported by saba.");
            // メインとなるファイルを開き現在の内容を読み込む
            let mut file = File::open(main_rs_path.clone())?;
            let mut file_contents = String::new();
            file.read_to_string(&mut file_contents)?;

            // mod_blockのパターン
            let re = Regex::new(self.modblock_pattern(&main_rs_path))?;

            if re.is_match(&file_contents) {
                // ファイル内にパターンが見つかった場合は置換
                let replaced_contents = re.replace_all(&file_contents, mod_block.as_str());
                let mut new_file = File::create(main_rs_path)?;
                new_file.write_all(replaced_contents.as_bytes())?;
            }
            else {

            }

        }

        Ok(mod_block)
    }
    fn upstream_modblock(&self, upstream: &Vec<Yaml>, mod_block: &mut String, tabs: &'a str) -> anyhow::Result<()> {
        let vec_default: &Vec<Yaml> = &vec![];
        let mut tabs = String::from(tabs);

        for u in upstream {
            let dirname = u["name"].as_str().unwrap();
            let upstream = u["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = u["codefile"].as_vec().unwrap_or(vec_default);

            mod_block.push_str(tabs.as_str());
            mod_block.push_str("pub mod ");
            mod_block.push_str(dirname);
            mod_block.push_str(" {\n");

            if !upstream.is_empty() {
                tabs.push_str("    ");
                self.upstream_modblock(
                    upstream,
                    mod_block,
                    &tabs,
                )?;
            }

            if !codefile.is_empty() {
                self.codefile_modblock(
                    codefile,
                    mod_block,
                    &tabs,
                )?;
            }

            mod_block.push_str(tabs.as_str());
            mod_block.push_str("}\n");
        };
        Ok(())
    }
    fn codefile_modblock(&self, codefile: &Vec<Yaml>, mod_block: &mut String, tabs: &'a str) -> anyhow::Result<()> {
        let mut tabs = String::from(tabs);
        tabs.push_str("    ");
        for f in codefile {
            let mut filename = f["name"].as_str().unwrap();

            // filenameがmod.rsの時はr#を追加する
            if filename == "mod" {
                filename = "r#mod";
            }

            mod_block.push_str(tabs.as_str());
            mod_block.push_str("pub mod ");
            mod_block.push_str(filename);
            mod_block.push_str(";\n");
        }

        Ok(())
    }
}

impl<'a> CodefileGenerator<'a> for GenerateRustFileUseCaseImpl {
    fn location_action(&self, manifest: &'a Manifest) -> anyhow::Result<()> {
        let root_path = manifest.root.get_path();
        let vec_default: &Vec<Yaml> = &vec![];

        for spec in manifest.spec.clone() {
            let mut workdir = PathBuf::from(&root_path);
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = spec["codefile"].as_vec().unwrap_or(vec_default);

            if location == "target" || location == ".cargo" {
                println!("[NOTE] target or .cargo is an important directory for your Rust project, so we will skip the operation.");
                continue;
            }
            else if location == "src" {
                workdir.push(location);
                fs::create_dir_all(workdir.clone())?;
            }
            else {
                workdir.push(location);

                // lib.rsの生成
                let mut lib_rs = workdir.clone();
                lib_rs.push("lib.rs");
                main_rs::gen(&lib_rs)?;

                // libプロジェクト向けなのでsrcをパスに追加
                workdir.push("src");
                fs::create_dir_all(workdir.clone())?;
            }

            if !upstream.is_empty() {
                self.upstream_action(workdir.clone(), upstream, &manifest)?;
            }

            if !codefile.is_empty() {
                self.codefile_action(workdir.clone(), codefile, &manifest)?;
            }
        }
        Ok(())
    }
    fn di_container_action(
        &self,
        wd: PathBuf,
        _: &'a Manifest,
    ) -> anyhow::Result<()> {
        let path_list_raw = PATH_LIST.lock().unwrap();
        let path_list = &*path_list_raw;
        let data = DiTmpl{
            imports: path_list,
        };

        let rendered_tmpl = data.render()?;

        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
    fn domain_model_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {

        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let data = DomainModelTmpl{
            fname: fname.as_str(),
            pkgname: pkgname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
    fn domain_repository_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = DomainRepositoryTmpl{
            fname: fname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
    fn infra_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = InfraTmpl{
            fname: fname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
    fn usecase_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let data = UseCaseTmpl{
            fname: fname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
    fn presentation_action(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let data = PresentationTmpl{
            fname: fname.as_str(),
            pkgname: pkgname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
    fn gen_file_default(
        &self,
        wd: PathBuf,
        manifest: &'a Manifest,
    ) -> anyhow::Result<()> {
        let fname = self.get_fname(wd.clone(), manifest).unwrap();
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let data = DefaultTmpl{
            fname: fname.as_str(),
            pkgname: pkgname.as_str(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
}
