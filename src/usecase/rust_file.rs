use crate::domain::model::manifest::Manifest;
use crate::usecase::interface::code_file_generator::{
    CodeFileGenerator,
    CodeFileGeneratorConfig,
};
use std::fs;

pub struct RustFileUseCase<'a> {
    pub repository: Manifest<'a>,

}

impl<'a> RustFileUseCase<'a> {
    pub fn new(repository: Manifest<'a>) -> Self {
        Self{
            repository,
        }
    }
    pub fn gen_file(&self) {
        let config = CodeFileGeneratorConfig::new(
            "rs",
            "./src",
        );
        self.location_action(self.repository, config);
    }
    pub get_file_contents(&self) -> &str {

    }
}

impl<'a> CodeFileGenerator<'a> for RustFileUseCase<'a> {
    fn location_action(
        &self,
        repository: Manifest<'a>,
        cnf: CodeFileGeneratorConfig<'a>,
    ) {
        let mut workdir = self.get_root_path(repository, cnf.default_root);

        for spec in repository.spec {
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap();
            let codefile = spec["codefile"].as_vec().unwrap();
            workdir.push_str("/");
            workdir.push_str(location);

            if !upstream.is_empty() {
                self.upstream_action(workdir, upstream, cnf);
            }

            if !codefile.is_empty() {
                self.codefile_action(workdir, codefile, cnf);
            }
        }
    }
    fn upstream_action(
        &self,
        mut workdir: String,
        upstream: &Vec<yaml_rust::Yaml>,
        cnf: CodeFileGeneratorConfig<'a>,
    ) {
        for u in upstream {
            let dirname = u["name"].as_str().unwrap();
            let upstream = u["upstream"].as_vec().unwrap();
            let codefile = u["codefile"].as_vec().unwrap();
            workdir.push_str("/");
            workdir.push_str(dirname);
            fs::create_dir_all(workdir);

            if !upstream.is_empty() {
                self.upstream_action(workdir, upstream, cnf);
            }

            if !codefile.is_empty() {
                self.codefile_action(workdir, codefile, cnf);
            }
        }
    }
    fn codefile_action(
        &self,
        mut workdir: String,
        codefile: &Vec<yaml_rust::Yaml>,
        cnf: CodeFileGeneratorConfig<'a>,
    ) {
        for f in codefile {
            let filename = f["name"].as_str().unwrap();

        }
    }
}
