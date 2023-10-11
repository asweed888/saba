use crate::domain::model::manifest::Manifest;
use crate::usecase::interface::CodeFileGenerator;

pub struct RustFileUseCase<'a> {
    pub repository: Manifest<'a>,
}

impl<'a> RustFileUseCase<'a> {
    pub fn new(repository: Manifest<'a>) -> Self {
        Self{ repository }
    }
    pub fn gen_file(&self) {
        self.location_action(self.repository);
    }
}

impl<'a> CodeFileGenerator<'a> for RustFileUseCase<'a> {
    fn location_action(&self, repository: Manifest<'a>) {
        let mut workdir = self.get_root_path(repository, "./src");

        for spec in repository.spec {
            let location = spec["location"].as_str().unwrap();
            let upstream = spec["upstream"].as_vec().unwrap();
            workdir.push_str("/");
            workdir.push_str(location);

            if !upstream.is_empty() {
                self.upstream_action(workdir, upstream);
            }
        }
    }
}
