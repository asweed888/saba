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
#[template(source = "from abc import ABC, abstractmethod


class I{{ u::to_title(fname) }}{{ u::to_title(pkgname) }}(ABC):
    @abstractmethod


class {{ u::to_title(fname) }}{{ u::to_title(pkgname) }}(I{{ u::to_title(fname) }}{{ u::to_title(pkgname) }}):
    def __init__(self, usecase):
        self.usecase = usecase
", ext = "txt")]
pub struct PresentationTmpl<'a> {
    pub fname: &'a str,
    pub pkgname: &'a str,
}


#[derive(Template)]
#[template(source = "class {{ u::to_title(fname) }}:
    def __init__(self):
", ext = "txt")]
pub struct DefaultTmpl<'a> {
    pub fname: &'a str,
}


pub fn di_tmpl() -> String {
    String::from("from abc import ABC, abstractmethod
from infrastructure.datasrore. import
from usecase. import
from presentation. import


class IDIContainer(ABC):
    @abstractmethod


class DIContainer(IDIContainer):
    def __init__(self):
")
}
