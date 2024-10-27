use crate::infrastructure::filesystem::manifest::ManifestRepository;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use yaml_rust::Yaml;
use anyhow::anyhow;

pub trait CodefileAct<'a> {
    fn gen_location(&self, repo: &'a ManifestRepository) -> anyhow::Result<()> {
        for spec in repo.manifest.spec.clone() {
            let mut workdir = repo.manifest.root.clone();
            let location = spec["location"].as_str().ok_or_else(|| anyhow!("Failed to get location from spec"))?;
            let upstream = spec["upstream"].as_vec().unwrap_or(&vec![]);
            let codefile = spec["codefile"].as_vec().unwrap_or(&vec![]);

            if location != "src" {
                workdir.push(location);
                fs::create_dir_all(workdir.clone())?;
            }

            if !codefile.is_empty() {
                self.gen_codefile(workdir.clone(), codefile, &repo)?;
            }
            if !upstream.is_empty() {
                self.gen_upstream(workdir.clone(), upstream, &repo)?;
            }
        }

        Ok(())
    }
    fn gen_upstream(&self, wd: PathBuf, upstream: &Vec<Yaml>, repo: &'a ManifestRepository) -> anyhow::Result<()> {
        for u in upstream {
            let mut workdir = wd.clone();
            let dirname = u["name"].as_str().ok_or_else(|| anyhow!("Failed to get name from upstream"))?;
            let upstream = u["upstream"].as_vec().unwrap_or(&vec![]);
            let codefile = u["codefile"].as_vec().unwrap_or(&vec![]);

            workdir.push(dirname);
            fs::create_dir_all(workdir.clone())?;

            if !codefile.is_empty() {
                self.gen_codefile(workdir.clone(), codefile, &repo)?;
            }
            if !upstream.is_empty() {
                self.gen_upstream(workdir.clone(), upstream, &repo)?;
            }
        }

        Ok(())
    }
    fn gen_codefile(&self, wd: PathBuf, codefile: &Vec<Yaml>, repo: &'a ManifestRepository) -> anyhow::Result<()> {
        let ext = repo.manifest.lang.ext();
        for f in codefile {
            let mut workdir = wd.clone();
            let filename = f["name"].as_str().ok_or_else(|| anyhow!("Failed to get name from codefile"))?;

            if repo.manifest.lang.is_generate_ignore(filename) {
                continue;
            }

            workdir.push(filename);
            self.set_ext(&mut workdir, ext)?;
            if !workdir.as_path().exists() {
                self.gen_codefile_main(workdir.clone(), &repo)?;
            }
        }

        Ok(())
    }
    fn gen_codefile_main(&self, wd: PathBuf, _repo: &'a ManifestRepository) -> anyhow::Result<()> {
        let wd_str = wd.to_str().ok_or_else(|| anyhow!("Failed to convert wd to str type"))?;
        File::create(wd_str)?;
        Ok(())
    }
    fn set_ext(&self, wd: &mut PathBuf, ext: &str) -> anyhow::Result<()> {
        let wd_str = wd.to_str().ok_or_else(|| anyhow!("Failed to convert wd to str type"))?;
        if wd_str.contains(".svelte") {
            wd.set_extension("svelte");
        }
        else if wd_str.contains(".tsx") {
            wd.set_extension("tsx");
        }
        else if wd_str.contains(".vue") {
            wd.set_extension("vue");
        }
        else {
            wd.set_extension(ext);
        }
        Ok(())
    }
    fn workdir_info(&self, wd: PathBuf, repo: &'a ManifestRepository) -> anyhow::Result<(String, String)> {
        let root = repo.manifest.root.clone();
        let fname = wd.file_stem()
            .ok_or_else(|| anyhow!("Failed to get file_stem"))?
            .to_str()
            .unwrap_or(root.as_str())
            .to_string();

        let parent = wd.parent()
            .ok_or_else(|| anyhow!("Failed to get parent directory"))?
            .file_name()
            .ok_or_else(|| anyhow!("Failed to get name of parent directory"))?
            .to_str()
            .unwrap_or("");



        let pkgname = match root.as_str() {
            "." => {
                if parent != "." {
                    parent.to_string()
                }
                else {
                    String::new()
                }
            }
            _ => {
                let replaced = root.replace("./", "");
                if parent != replaced.as_str() {
                    parent.to_string()
                }
                else {
                    String::new()
                }
            }
        };

        Ok((fname, pkgname))
    }
}
