use askama::Template;
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

pub struct GenerateRustFileUseCaseImpl {
    manifest: Manifest,
}

impl<'a> GenerateRustFileUseCaseImpl {
    pub fn new(manifest: Manifest) -> Self {
        Self{ manifest }
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.location_action(&self.manifest)?;
        self.save_modblock()?;

        Ok(())
    }
    fn upstream_modblock_with_path(
        &self,
        upstream: &Vec<Yaml>,
        mod_block: &mut String,
        path: &'a PathBuf,
        tabs: &'a str
    ) -> anyhow::Result<()> {
        let path = path.to_str().unwrap().to_string();
        let vec_default: &Vec<Yaml> = &vec![];
        let tabs = String::from(tabs);

        for u in upstream {
            let dirname = u["name"].as_str().unwrap();
            let upstream = u["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = u["codefile"].as_vec().unwrap_or(vec_default);

            if path.contains("lib.rs") {
                mod_block.push_str("pub ");
            }
            mod_block.push_str("mod ");
            mod_block.push_str(dirname);
            mod_block.push_str(" {\n");

            if !upstream.is_empty() {
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
    fn codefile_modblock_with_path(
        &self,
        codefile: &Vec<Yaml>,
        mod_block: &mut String,
        path: &'a PathBuf,
        tabs: &'a str
    ) -> anyhow::Result<()> {
        let path = path.to_str().unwrap().to_string();
        let mut tabs = String::from(tabs);
        tabs.push_str("    ");
        for f in codefile {
            let mut filename = f["name"].as_str().unwrap();


            // filenameがmod.rsの時はr#を追加する
            if filename == "main" {
                println!("[WORKING] Do not include main.rs in the list of codefiles.");
            }
            else if filename == "mod" {
                filename = "r#mod";
            }

            if path.contains("lib.rs") {
                mod_block.push_str("pub ");
            }
            mod_block.push_str("mod ");
            mod_block.push_str(filename);
            mod_block.push_str(";\n");
        }

        Ok(())
    }
}

impl<'a> ModblockHandler<'a> for GenerateRustFileUseCaseImpl {
    fn save_modblock(&self) -> anyhow::Result<()> {
        let mut mod_block = String::new();
        let vec_default: &Vec<Yaml> = &vec![];

        for (_, spec) in self.manifest.spec.iter().enumerate() {
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = spec["codefile"].as_vec().unwrap_or(vec_default);

            let main_rs_path = main_rs::path(&PathBuf::from(location))?;
            let mut tabs = String::new();

            if !upstream.is_empty() {
                self.upstream_modblock_with_path(
                    upstream,
                    &mut mod_block,
                    &main_rs_path,
                    &tabs,
                )?;
                tabs = String::new();
            }

            if !codefile.is_empty() {
                self.codefile_modblock_with_path(
                    codefile,
                    &mut mod_block,
                    &main_rs_path,
                    &tabs,
                )?;
            }

            mod_block.push_str("// Automatically exported by saba.");
            // メインとなるファイルを開き現在の内容を読み込む
            let mut file = File::open(main_rs_path.clone())?;
            let mut file_contents = String::new();
            file.read_to_string(&mut file_contents)?;

            // mod_blockのパターン
            let re = Regex::new(self.modblock_pattern())?;

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
            mod_block = String::new();
        }


        Ok(())
    }
    fn upstream_modblock(&self, upstream: &Vec<Yaml>, mod_block: &mut String, tabs: &'a str) -> anyhow::Result<()> {
        let vec_default: &Vec<Yaml> = &vec![];
        let mut tabs = String::from(tabs);
        tabs.push_str("    ");

        for u in upstream {
            let dirname = u["name"].as_str().unwrap();
            let upstream = u["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = u["codefile"].as_vec().unwrap_or(vec_default);

            mod_block.push_str(tabs.as_str());
            mod_block.push_str("pub mod ");
            mod_block.push_str(dirname);
            mod_block.push_str(" {\n");

            if !upstream.is_empty() {
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
                // main.rsへのパスを作成
                let mut main_rs_path = workdir.clone();
                main_rs_path.push("main.rs");

                // srcまでのパスを作成
                fs::create_dir_all(workdir.clone())?;

                // main.rsを作成
                main_rs::gen(&main_rs_path)?;
            }
            else {
                workdir.push(location);
                // lib.rsへのパスを作成
                let mut lib_rs_path = workdir.clone();
                lib_rs_path.push("lib.rs");

                // lib/srcまでのディレクトリを作成
                workdir.push("src");
                fs::create_dir_all(workdir.clone())?;

                // lib.rsを作成
                main_rs::gen(&lib_rs_path)?;
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
        let pkgname = self.get_pkgname(wd.clone(), manifest).unwrap();
        let data = UseCaseTmpl{
            fname: fname.as_str(),
            pkgname: pkgname.as_str(),
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
            wd: wd.to_str().unwrap(),
        };

        let rendered_tmpl = data.render()?;
        let mut file = File::create(wd.to_str().unwrap())?;

        file.write_all(rendered_tmpl.as_bytes())?;

        Ok(())
    }
}
