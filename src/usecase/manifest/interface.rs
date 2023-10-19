use yaml_rust::Yaml;
use crate::domain::manifest::entity::Manifest;

pub trait TGenerateFileUseCase<'a> {
    fn location_action(&self, manifest: &'a Manifest) {
        let mut workdir = manifest.root.path().to_string();

        for spec in manifest.spec {
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap();
            let codefile = spec["codefile"].as_vec().unwrap();

            workdir.push_str("/");
            workdir.push_str(location);

            if !upstream.is_empty() {
                self.upstream_action(workdir, upstream, &manifest);
            }

            if !codefile.is_empty() {

            }
        }
    }
    fn upstream_action(
        &self,
        mut workdir: String,
        upstream: &Vec<Yaml>,
        manifest: &'a Manifest
    ) {

    }
    fn codefile_action(
        &self,
        mut workdir: String,
        codefile: &Vec<Yaml>,
        manifest: &'a Manifest
    ) {

    }
}
