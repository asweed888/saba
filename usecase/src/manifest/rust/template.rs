use askama::Template;
use crate::manifest::utils;
use crate::manifest::rust::utils as rs_utils;

#[derive(Template)]
#[template(source = "pub struct {{ utils::default_struct(pkgname, fname) }} {}", ext = "txt")]
pub struct DomainModelTmpl<'a> {
    pub fname: &'a str,
    pub pkgname: &'a str,
}


#[derive(Template)]
#[template(source = "pub trait {{ utils::to_title(fname) }}Repository {}", ext = "txt")]
pub struct DomainRepositoryTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "use crate::domain::model::{{ fname }}::entity::{{ utils::to_title(fname) }};
use crate::domain::repository::{{ fname }}::{{ utils::to_title(fname) }}Repository;


pub struct {{ utils::to_title(fname) }}RepositoryImpl;

impl {{ utils::to_title(fname) }}RepositoryImpl {
    pub fn new() -> Self {
        Self{}
    }
}

impl {{ utils::to_title(fname) }}Repository for {{ utils::to_title(fname) }}RepositoryImpl {}
", ext = "txt")]
pub struct InfraTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "use crate::domain::model::{{ fname }}::entity::{{ utils::to_title(fname) }};
use crate::domain::repository::{{ fname }}::{{ utils::to_title(fname) }}Repository;


pub struct {{ utils::to_title(fname) }}UseCaseImpl<R>;
where
    R: {{ utils::to_title(fname) }}Repository,
{
    pub repository: R,
}

impl<R> {{ utils::to_title(fname) }}UseCaseImpl<R>;
where
    R: {{ utils::to_title(fname) }}Repository,
{
    pub fn new(repository: R) -> Self {
        Self{ repository }
    }
}
", ext = "txt")]
pub struct UseCaseTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "use crate::domain::model::{{ fname }}::entity::{{ utils::to_title(fname) }};
use crate::domain::repository::{{ fname }}::{{ utils::to_title(fname) }}Repository;
use crate::usecase::{{ fname }}::{{ utils::to_title(fname) }}UseCaseImpl;


pub struct {{ utils::to_title(fname) }}{{ utils::to_title(pkgname) }}<R>
where
    R: {{ utils::to_title(fname) }}Repository,
{
    pub usecase: {{ utils::to_title(fname) }}UseCaseImpl<R>,
}

impl<R> {{ utils::to_title(fname) }}{{ utils::to_title(pkgname) }}<R>
where
    R: {{ utils::to_title(fname) }}Repository,
{
    pub fn new(usecase: {{ utils::to_title(fname) }}UseCaseImpl<R>) -> Self {
        Self{ usecase }
    }
}
", ext = "txt")]
pub struct PresentationTmpl<'a> {
    pub fname: &'a str,
    pub pkgname: &'a str,
}


#[derive(Template)]
#[template(source = "{% for import in imports -%}
{% if !rs_utils::contains_di_str(import) -%}
use {{ rs_utils::crate_path(import) }};
{% endif -%}
{% endfor %}
pub struct App {}

pub struct DIContainer {}

impl DIContainer {
    pub fn new() -> Self {
        Self{}
    }
    pub fn new_app(&self) -> App {
        App{}
    }
}
", ext = "txt")]
pub struct DiTmpl<'a> {
    pub imports: &'a Vec<String>,
}

#[derive(Template)]
#[template(source = "pub struct {{ utils::default_struct(pkgname, fname) }} {}", ext = "txt")]
pub struct DefaultTmpl<'a> {
    pub fname: &'a str,
    pub pkgname: &'a str,
}