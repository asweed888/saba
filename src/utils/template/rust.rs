use askama::Template;
use crate::utils::template::prelude;

#[derive(Template)]
#[template(source = "pub struct {{ prelude::to_title(fname) }} {}", ext = "txt")]
pub struct DomainModelTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "pub struct {{ prelude::to_title(fname) }}Repository {}", ext = "txt")]
pub struct DomainRepositoryTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "use crate::domain::model::{{ fname }}::{{ prelude::to_title(fname) }};
use crate::domain::repository::{{ fname }}::{{ prelude::to_title(fname) }}Repository;


impl {{ prelude::to_title(fname) }}Repository {
    pub fn new() -> Self {
        Self{}
    }
}
", ext = "txt")]
pub struct InfraTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "use crate::domain::model::{{ fname }}::{{ prelude::to_title(fname) }};
use crate::domain::repository::{{ fname }}::{{ prelude::to_title(fname) }}Repository;


pub struct {{ prelude::to_title(fname) }}UseCase {
    pub repository: {{ prelude::to_title(fname) }}Repository,
}

impl {{ prelude::to_title(fname) }}UseCase {
    pub fn new(repository: {{ prelude::to_title(fname) }}Repository) -> Self {
        Self{ repository }
    }
}
", ext = "txt")]
pub struct UseCaseTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "use crate::domain::model::{{ fname }}::{{ prelude::to_title(fname) }};
use crate::usecase::{{ fname }}::{{ prelude::to_title(fname) }}UseCase;


pub struct {{ prelude::to_title(fname) }}{{ prelude::to_title(pkgname) }} {
    usecase: {{ prelude::to_title(fname) }}UseCase,
}

impl {{ prelude::to_title(fname) }}{{ prelude::to_title(pkgname) }} {
    pub fn new(usecase: {{ prelude::to_title(fname) }}UseCase) -> Self {
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