use askama::Template;
use crate::usecase::manifest::utils;

#[derive(Template)]
#[template(source = "pub struct {{ utils::to_title(fname) }} {}", ext = "txt")]
pub struct DomainModelTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "pub struct {{ utils::to_title(fname) }}Repository {}", ext = "txt")]
pub struct DomainRepositoryTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "use crate::domain::model::{{ fname }}::{{ utils::to_title(fname) }};
use crate::domain::repository::{{ fname }}::{{ utils::to_title(fname) }}Repository;


impl {{ utils::to_title(fname) }}Repository {
    pub fn new() -> Self {
        Self{}
    }
}
", ext = "txt")]
pub struct InfraTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "use crate::domain::model::{{ fname }}::{{ utils::to_title(fname) }};
use crate::domain::repository::{{ fname }}::{{ utils::to_title(fname) }}Repository;


pub struct {{ utils::to_title(fname) }}UseCase {
    pub repository: {{ utils::to_title(fname) }}Repository,
}

impl {{ utils::to_title(fname) }}UseCase {
    pub fn new(repository: {{ utils::to_title(fname) }}Repository) -> Self {
        Self{ repository }
    }
}
", ext = "txt")]
pub struct UseCaseTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "use crate::domain::model::{{ fname }}::{{ utils::to_title(fname) }};
use crate::usecase::{{ fname }}::{{ utils::to_title(fname) }}UseCase;


pub struct {{ utils::to_title(fname) }}{{ utils::to_title(pkgname) }} {
    usecase: {{ utils::to_title(fname) }}UseCase,
}

impl {{ utils::to_title(fname) }}{{ utils::to_title(pkgname) }} {
    pub fn new(usecase: {{ utils::to_title(fname) }}UseCase) -> Self {
        Self{ usecase }
    }
}
", ext = "txt")]
pub struct PresentationTmpl<'a> {
    pub fname: &'a str,
    pub pkgname: &'a str,
}


#[derive(Template)]
#[template(source = "

pub struct App {}

pub struct DIContainer {}

impl DIContainer {
    pub fn new() -> Self {
        Self{}
    }
    pub fn new_app() -> App {
        App{}
    }
}
", ext = "txt")]
pub struct DiTmpl {}


#[derive(Template)]
#[template(source = "", ext = "txt")]
pub struct DefaultTmpl {}
