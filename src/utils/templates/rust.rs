use askama::Template;
use crate::utils::generic as utils;
use crate::utils::rust as rs_utils;


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


pub struct {{ utils::to_title(fname) }}RepositoryImpl{}

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

pub fn create<R>(repo: &R)
where
    R: {{ utils::to_title(fname) }}Repository
{}

pub fn get_all<R>(repo: &R)
where
    R: {{ utils::to_title(fname) }}Repository
{}

pub fn update<R>(repo: &R)
where
    R: {{ utils::to_title(fname) }}Repository
{}

pub fn delete<R>(repo: &R)
where
    R: {{ utils::to_title(fname) }}Repository
{}

", ext = "txt")]
pub struct UseCaseTmpl<'a> {
    pub fname: &'a str,
    pub pkgname: &'a str,
}


#[derive(Template)]
#[template(source = "use crate::domain::model::{{ fname }}::entity::{{ utils::to_title(fname) }};
use crate::domain::repository::{{ fname }}::{{ utils::to_title(fname) }}Repository;
use crate::usecase::{{ fname }}::{{ utils::to_title(fname) }}UseCaseImpl;


pub struct {{ utils::to_title(fname) }}{{ utils::to_title(pkgname) }}Impl<R>
where
    R: {{ utils::to_title(fname) }}Repository,
{
    pub usecase: {{ utils::to_title(fname) }}UseCaseImpl<R>,
}

impl<R> {{ utils::to_title(fname) }}{{ utils::to_title(pkgname) }}Impl<R>
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
#[template(source = "{%- if rs_utils::contains_traits_str(wd) -%}
pub trait Trait {}
{%- else if rs_utils::contains_act_str(wd) -%}
pub trait Act {}
{%- else -%}
pub struct {{ utils::default_struct(pkgname, fname) }} {}
{%- endif -%}
", ext = "txt")]
pub struct DefaultTmpl<'a> {
    pub fname: &'a str,
    pub pkgname: &'a str,
    pub wd: &'a str,
}
