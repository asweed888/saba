use askama::Template;
use crate::usecase::manifest::utils as u;

#[derive(Template)]
#[template(source = "class {{u::to_title(fname)}}:
    def __init__(self):
", ext = "txt")]
pub struct DomainModelTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "from abc import ABC, abstractmethod


class I{{u::to_title(fname)}}Repository(ABC):
    @abstractmethod
", ext = "txt")]
pub struct DomainRepositoryTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "from domain.repository.{{ fname }} import I{{ u::to_title(fname) }}Repository


class {{ u::to_title(fname) }}Repository(I{{ u::to_title(fname) }}Repository):
    def __init__(self):
", ext = "txt")]
pub struct InfraTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "from abc import ABC, abstractmethod


class I{{ u::to_title(fname) }}UseCase(ABC):
    @abstractmethod


class {{ u::to_title(fname) }}UseCase(I{{ u::to_title(fname) }}UseCase):
    def __init__(self, repository):
        self.repository = repository
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
#[template(source = "pub struct {{ utils::to_title(fname) }} {}", ext = "txt")]
pub struct DefaultTmpl<'a> {
    pub fname: &'a str,
}
