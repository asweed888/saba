use askama::Template;
use crate::utils::generic as utils;

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

type {{ utils::to_title(fname) }}RepositoryAct interface {}
", ext = "txt")]
pub struct DomainRepositoryTmpl<'a> {
    pub pkgname: &'a str,
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "package {{ pkgname }}

type {{ fname }}Repository struct {}

func New{{ utils::to_title(fname) }}Repository() repository.{{ utils::to_title(fname) }}RepositoryAct {
    return &{{ fname }}Repository{}
}
", ext = "txt")]
pub struct InfraTmpl<'a> {
    pub pkgname: &'a str,
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "package {{ pkgname }}

type {{ utils::to_title(fname) }}UseCaseAct interface {}

type {{ fname }}UseCase struct {
    repository.{{ utils::to_title(fname) }}RepositoryAct
}

func New{{ utils::to_title(fname) }}UseCase(r repository.{{ utils::to_title(fname) }}RepositoryAct) {{ utils::to_title(fname) }}UseCaseAct {
    return &{{ fname }}UseCase{r}
}
", ext = "txt")]
pub struct UseCaseTmpl<'a> {
    pub pkgname: &'a str,
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "package {{ pkgname }}

type {{ utils::to_title(fname) }}HandlerAct interface {}

type {{ fname }}Handler struct {
    usecase.{{ utils::to_title(fname) }}UseCaseAct
}

func New{{ utils::to_title(fname) }}Handler(u usecase.{{ utils::to_title(fname) }}UseCaseAct) {{ utils::to_title(fname) }}HandlerAct {
    return &{{ fname }}Handler{u}
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

type DiContainerAct interface {}

type diContainer struct {}

func NewDiContainer() DiContainerAct {
    return &diContainer{}
}
")
}
