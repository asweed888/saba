use askama::Template;
use sabacan::codefile::template::utils as u;

#[derive(Template)]
#[template(source = "export class {{ u::to_title(fname) }} {
  constructor() {}
}
", ext = "txt")]
pub struct DomainModelTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "import { {{ u::to_title(fname) }} } from '../model/{{ fname }}'

export interface I{{ u::to_title(fname) }}Repository {}
", ext = "txt")]
pub struct DomainRepositoryTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "import { {{ u::to_title(fname) }} } from '../../domain/model/{{ fname }}'
import { I{{ u::to_title(fname) }}Repository } from '../../domain/repository/{{ fname }}'

export class {{ u::to_title(fname) }}Repository implements I{{ u::to_title(fname) }}Repository {
  constructor() {}
}
", ext = "txt")]
pub struct InfraTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "import { {{ u::to_title(fname) }} } from '../domain/model/{{ fname }}'
import { I{{ u::to_title(fname) }}Repository } from '../domain/repository/{{ fname }}'

export interface I{{ u::to_title(fname) }}UseCase {}

export class {{ u::to_title(fname) }}UseCase implements I{{ u::to_title(fname) }}UseCase {
  repository: I{{ u::to_title(fname) }}Repository

  constructor(repository: I{{ u::to_title(fname) }}Repository) {
    this.repository = repository
  }
}
", ext = "txt")]
pub struct UseCaseTmpl<'a> {
    pub fname: &'a str,
}


#[derive(Template)]
#[template(source = "import { {{ u::to_title(fname) }} } from '../../../domain/model/{{ fname }}'
import { I{{ u::to_title(fname) }}UseCase } from '../../../usecase/{{ fname }}'

export interface I{{ u::to_title(fname) }}{{ u::to_title(pkgname) }} {}

export class {{ u::to_title(fname) }}{{ u::to_title(pkgname) }} implements I{{ u::to_title(fname) }}{{ u::to_title(pkgname) }} {
  usecase: I{{ u::to_title(fname) }}UseCase

  constructor(usecase: I{{ u::to_title(fname) }}UseCase) {
    this.usecase = usecase
  }
}
", ext = "txt")]
pub struct PresentationTmpl<'a> {
    pub fname: &'a str,
    pub pkgname: &'a str,
}


#[derive(Template)]
#[template(source = "export class {{ u::to_title(fname) }} {
  constructor() {}
}
", ext = "txt")]
pub struct DefaultTmpl<'a> {
    pub fname: &'a str,
}


pub fn di_tmpl() -> String {
    String::from("import {  } from '../domain/repository'
import {  } from '../infrastructure/datastore'
import {  } from '../presentation'

interface IDIContainer {}

export class DIContainer implements IDIContainer {
  constructor() {}
}
")
}
