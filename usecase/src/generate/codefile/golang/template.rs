use askama::Template;
use sabacan::codefile::template::utils;

#[derive(Template)]
#[template(source = "package {{ pkgname }}

type {{ utils::to_title(fname) }} struct {}
", ext = "txt")]
pub struct DomainModelTmpl<'a> {
    pub pkgname: &'a str,
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "package {{ pkgname }}

type {{ utils::to_title(fname) }}Repository interface {}
", ext = "txt")]
pub struct DomainRepositoryTmpl<'a> {
    pub pkgname: &'a str,
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "package {{ pkgname }}

type {{ fname }}Repository struct {}

func New{{ utils::to_title(fname) }}Repository() repository.{{ utils::to_title(fname) }}Repository {
    return &{{ fname }}Repository{}
}
", ext = "txt")]
pub struct InfraTmpl<'a> {
    pub pkgname: &'a str,
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "package {{ pkgname }}

type {{ utils::to_title(fname) }}UseCase interface {}

type {{ fname }}UseCase struct {
    repository.{{ utils::to_title(fname) }}Repository
}

func New{{ utils::to_title(fname) }}UseCase(r repository.{{ utils::to_title(fname) }}Repository) {{ utils::to_title(fname) }}UseCase {
    return &{{ fname }}UseCase{r}
}
", ext = "txt")]
pub struct UseCaseTmpl<'a> {
    pub pkgname: &'a str,
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "package {{ pkgname }}

type {{ utils::to_title(fname) }}{{ utils::to_title(pkgname) }} interface {}

type {{ fname }}{{ utils::to_title(pkgname) }} struct {
    usecase.{{ utils::to_title(fname) }}UseCase
}

func New{{ utils::to_title(fname) }}{{ utils::to_title(pkgname) }}(u usecase.{{ utils::to_title(fname) }}UseCase) {{ utils::to_title(fname) }}{{ utils::to_title(pkgname) }} {
    return &{{ fname }}{{ utils::to_title(pkgname) }}{u}
}
", ext = "txt")]
pub struct PresentationTmpl<'a> {
    pub pkgname: &'a str,
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "package {{ pkgname }}", ext = "txt")]
pub struct DefaultTmpl<'a> {
    pub pkgname: &'a str,
}


pub fn di_tmpl() -> String {
    String::from("package di

type DiContainer interface {}

type diContainer struct {}

func NewDiContainer() DiContainer {
    return &diContainer{}
}
")
}