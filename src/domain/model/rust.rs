use askama::Template;

#[derive(Template)]
#[template(source = "pub struct {{ fname }}", ext = "txt")];
pub struct RustDomainModel {

}
