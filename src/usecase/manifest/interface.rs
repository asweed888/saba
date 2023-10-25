use yaml_rust::Yaml;
use crate::domain::manifest::entity::Manifest;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

// impl<'a> WorkDir<'a> {
//     pub fn new(root_path: &'a str) -> Self {
//         Self{
//             path: PathBuf::from(root_path.clone()),
//             root: root_path,
//         }
//     }
//     pub fn path_join(&mut self, s: &'a str) {
//         self.path.join(s);
//     }
//     pub fn fname(&mut self) -> Option<String> {
//         Some(
//             self.path
//             .file_name()
//             .unwrap()
//             .to_str()
//             .unwrap_or(
//                 self.root,
//             )
//             .to_string()
//         )
//     }
//     pub fn pkgname(&mut self) -> Option<String> {
//         let parent = self.path
//             .parent()
//             .unwrap()
//             .to_str()
//             .unwrap_or("");
//         match self.root {
//             "." => {
//                 if parent != "." {
//                     Some(parent.to_string())
//                 }
//                 else {
//                     None
//                 }
//             }
//             _ => {
//                 let root = self.root.replace("./", "");
//                 if parent != root.as_str() {
//                     Some(parent.to_string())
//                 }
//                 else {
//                     None
//                 }
//             }
//         }
//     }
//     pub fn path_contains(&mut self, s: &'a str) -> bool {
//         self.path.to_str().unwrap().contains(s)
//     }
// }


pub trait TGenerateFileUseCase<'a> {
    fn location_action(&self, manifest: &'a Manifest) -> Result<()> {
        let root_path = manifest.root.get_path();
        let mut workdir = PathBuf::from(root_path);
        let vec_default: &Vec<Yaml> = &vec![];

        for spec in manifest.spec.clone() {
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = spec["codefile"].as_vec().unwrap_or(vec_default);

            workdir.push(location);

            if !upstream.is_empty() {
                self.upstream_action(workdir.clone(), upstream, &manifest)?;
            }

            if !codefile.is_empty() {
                self.codefile_action(workdir.clone(), codefile, &manifest)?;
            }
        }
        Ok(())
    }
    fn upstream_action(
        &self,
        mut workdir: PathBuf,
        upstream: &Vec<Yaml>,
        manifest: &'a Manifest
    ) -> Result<()> {
        let vec_default: &Vec<Yaml> = &vec![];

        for u in upstream {
            let dirname = u["name"].as_str().unwrap();
            let upstream = u["upstream"].as_vec().unwrap_or(vec_default);
            let codefile = u["codefile"].as_vec().unwrap_or(vec_default);

            workdir.push(dirname);

            // fs::create_dir_all(workdir.clone())?;

            if !upstream.is_empty() {
                self.upstream_action(workdir.clone(), upstream, manifest)?;
            }

            if !codefile.is_empty() {
                self.codefile_action(workdir.clone(), codefile, manifest)?;
            }
        }
        Ok(())
    }
    fn codefile_action(
        &self,
        mut workdir: PathBuf,
        codefile: &Vec<Yaml>,
        manifest: &'a Manifest
    ) -> Result<()> {
        let ext = manifest.lang.ext().as_str();
        for f in codefile {
            let filename = f["name"].as_str().unwrap();

            workdir.push(filename);
            workdir.set_extension(ext);
            let path = workdir.to_str().unwrap();

            println!("workdir: {}", path);

            if self.is_ddd_enabled(manifest) {
                if path.contains("domain/model") {
                    self.domain_model_action(workdir.clone())?;
                }
                else if path.contains("domain/repository") {
                    self.domain_repository_action(workdir.clone())?;
                }
                else if path.contains("/infrastructure") {
                    self.infra_action(workdir.clone())?;
                }
                else if path.contains("/usecase") {
                    self.usecase_action(workdir.clone())?;
                }
                else if path.contains("presentation") {
                    self.presentation_action(workdir.clone())?;
                }
            }
            else {
                self.gen_file_default(workdir.clone())?;
            }
        }
        Ok(())
    }
    fn is_ddd_enabled(&self, manifest: &'a Manifest) -> bool {
        manifest.arch.is_ddd()
        && manifest.lang.name().as_str() != "bash"
    }
    fn domain_model_action(
        &self,
        workdir: PathBuf,
    ) -> Result<()> {
        println!("generating file: {}", workdir.to_str().unwrap());
        Ok(())
    }
    fn domain_repository_action(
        &self,
        workdir: PathBuf,
    ) -> Result<()> {
        println!("generating file: {}", workdir.to_str().unwrap());
        Ok(())
    }
    fn infra_action(&self,
        workdir: PathBuf,
    ) -> Result<()> {
        println!("generating file: {}", workdir.to_str().unwrap());
        Ok(())
    }
    fn usecase_action(&self,
        workdir: PathBuf,
    ) -> Result<()> {
        println!("generating file: {}", workdir.to_str().unwrap());
        Ok(())
    }
    fn presentation_action(&self,
        workdir: PathBuf,
    ) -> Result<()> {
        println!("generating file: {}", workdir.to_str().unwrap());
        Ok(())
    }
    fn gen_file_default(&self,
        workdir: PathBuf,
    ) -> Result<()> {
        println!("generating file: {}", workdir.to_str().unwrap());
        Ok(())
    }
}
