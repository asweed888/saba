use crate::infrastructure::filesystem::manifest::ManifestRepository;
use crate::usecase::gen_file::codefile::act::CodefileAct;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use askama::Template;
use super::template::*;
use anyhow::anyhow;

pub struct Python<'a> {
    repo: &'a ManifestRepository,
}

impl<'a> Python<'a> {
    pub fn new(repo: &'a ManifestRepository) -> anyhow::Result<Self> {
        Ok(Self{ repo })
    }
    pub fn gen_file(&self) -> anyhow::Result<()> {
        self.gen_location(&self.repo)?;
        Ok(())
    }
}

impl<'a> CodefileAct<'a> for Python<'a> {
    fn gen_codefile_main(&self, wd: PathBuf, repo: &'a ManifestRepository) -> anyhow::Result<()> {
        let path = wd.to_str().ok_or_else(|| anyhow!("Failed to convert wd to str type"))?;

        let is_ddd = repo.manifest.arch.is_ddd();
        let (fname, pkgname) = self.workdir_info(wd.clone(), &repo)?;
        let (fname, pkgname) = { (fname.as_str(), pkgname.as_str()) };

        if is_ddd {
            if path.contains("/domain/model/") {
                let data = DomainModelTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(path)?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/domain/repository/") {
                let data = DomainRepositoryTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(path)?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/infrastructure/") {
                let data = InfraTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(path)?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/usecase/") {
                let data = UseCaseTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(path)?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else if path.contains("/presentation/") {
                let data = PresentationTmpl{fname, pkgname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(path)?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
            else {
                let data = DefaultTmpl{fname};
                let rendered_tmpl = data.render()?;
                let mut file = File::create(path)?;
                file.write_all(rendered_tmpl.as_bytes())?;
            }
        }
        else {
            let data = DefaultTmpl{fname};
            let rendered_tmpl = data.render()?;
            let mut file = File::create(path)?;
            file.write_all(rendered_tmpl.as_bytes())?;
        }

        Ok(())
    }
}
