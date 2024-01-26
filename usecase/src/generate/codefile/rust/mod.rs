use askama::Template;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use regex::Regex;
use yaml_rust::Yaml;
use sabacan::manifest::domain::model::entity::Manifest;
use sabacan::manifest::usecase::generate::codefile::{CodefileGenerator, PATH_LIST};
use crate::generate::codefile::rust::template::{
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
        self.write_mod_block4main_rs()?;
        Ok(())
    }
    fn write_mod_block4main_rs(&self) -> anyhow::Result<()> {
        let main_rs_path = self.get_main_rs_path()?;
        let mod_block = self.mod_block(main_rs_path.to_str().unwrap())?;
        let mut file = File::open(main_rs_path.to_str().unwrap())?;

        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let re = Regex::new(self.mod_block_regx(main_rs_path.to_str().unwrap()))?;

        if re.is_match(&file_contents) {
            // ファイル内にパターンが見つかった場合は置換
            let replaced_contents = re.replace_all(&file_contents, mod_block.as_str());

            let mut new_file = File::create(main_rs_path.to_str().unwrap())?;
            new_file.write_all(replaced_contents.as_bytes())?;
        } else {
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
    fn get_main_rs_path(&self) -> anyhow::Result<PathBuf> {
        let root = self.manifest.root.get_path();
        let fpath1 = PathBuf::from(root.to_string() + "/main.rs");
        let fpath2 = PathBuf::from(root.to_string() + "/lib.rs");
        if fpath1.as_path().exists() {
            return Ok(fpath1)
        }

        if fpath2.as_path().exists() {
            return Ok(fpath2)
        }

        let mut file = File::create(fpath1.to_str().unwrap())?;
        file.write_all("".as_bytes())?;

        Ok(fpath1)
    }
    fn mod_block_regx(&self, main_rs_path: &'a str) -> &str {
        if main_rs_path.contains("lib.rs") {
            r"pub mod[\s\S]*//.*Automatically exported by saba\."
        }
        else {
            r"mod[\s\S]*//.*Automatically exported by saba\."
        }
    }
    fn mod_block(&self, main_rs_path: &'a str) -> anyhow::Result<String> {
        let mut file_contents = String::new();
        let vec_default: &Vec<Yaml> = &vec![];

        for (idx, spec) in self.manifest.spec.iter().enumerate() {
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = spec["codefile"].as_vec().unwrap_or(vec_default);
            let mut tabs = String::new();

            // lib.rsの場合は先頭にpubをつける
            if main_rs_path.contains("lib.rs") {
                file_contents += "pub ";
            }
            file_contents += "mod ";
            file_contents += location;
            file_contents += " {\n";

            if !upstream.is_empty() {
                self.upstream_mod_block(
                    upstream,
                    &mut file_contents,
                    &tabs,
                )?;
                tabs = String::new();
            }

            if !codefile.is_empty() {
                self.codefile_mod_block(
                    codefile,
                    &mut file_contents,
                    &tabs,
                )?;
            }

            if idx == self.manifest.spec.len() - 1 {
                file_contents.push_str("} // Automatically exported by saba.");
            }
            else {
                file_contents.push_str("}\n");
            }
        }

        Ok(file_contents)
    }
    fn upstream_mod_block(
        &self,
        upstream: &Vec<Yaml>,
        file_contents: &mut String,
        t: &'a str,
    ) -> anyhow::Result<()> {
        let vec_default: &Vec<Yaml> = &vec![];
        let mut tabs = String::from(t);
        tabs.push_str("    ");

        for u in upstream {
            let dirname = u["name"].as_str().unwrap();
            let upstream = u["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = u["codefile"].as_vec().unwrap_or(vec_default);

            file_contents.push_str(tabs.as_str());
            file_contents.push_str("pub mod ");
            file_contents.push_str(dirname);
            file_contents.push_str(" {\n");

            if !upstream.is_empty() {
                self.upstream_mod_block(
                    upstream,
                    file_contents,
                    &tabs,
                )?;
            }

            if !codefile.is_empty() {
                self.codefile_mod_block(
                    codefile,
                    file_contents,
                    &tabs,
                )?;
            }

            file_contents.push_str(tabs.as_str());
            file_contents.push_str("}\n");
        }
        Ok(())
    }
    fn codefile_mod_block(
        &self,
        codefile: &Vec<Yaml>,
        file_contents: &mut String,
        t: &'a str,
    ) -> anyhow::Result<()> {
        let mut tabs = String::from(t);
        tabs.push_str("    ");
        for f in codefile {
            let mut filename = f["name"].as_str().unwrap();

            // filenameがmod.rsの時はr#を追加する
            if filename == "mod" {
                filename = "r#mod";
            }

            file_contents.push_str(tabs.as_str());
            file_contents.push_str("pub mod ");
            file_contents.push_str(filename);
            file_contents.push_str(";\n");
        }

        Ok(())
    }
}


impl<'a> CodefileGenerator<'a> for GenerateRustFileUseCaseImpl {
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