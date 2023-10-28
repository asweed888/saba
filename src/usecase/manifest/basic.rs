use crate::domain::manifest::entity::{
    Manifest,
    ManifestRepository,
};
use anyhow::Result;
use askama::Template;


pub struct ManifestUseCase {
    pub repository: ManifestRepository,
}

impl ManifestUseCase {
    pub fn new(repository: ManifestRepository) -> Self {
        Self{
            repository,
        }
    }
    pub fn load(&self) -> Result<Manifest> {
        self.repository.load()
    }
}

#[derive(Template)]
#[template(source = "lang: {{ lang }}
{% if is_ddd -%}
arch: ddd
{% endif -%}
spec:
{% if !is_ddd -%}
- location: repository
  codefile:
    - name: character
{% else -%}
- location: domain
  upstream:
    - name: model
      codefile:
        - name: character

    - name: repository
      codefile:
        - name: character


- location: infrastructure
  upstream:
    - name: datastore
      codefile:
        - name: character


- location: usecase
  codefile:
    - name: character


- location: presentation
  upstream:
    - name: http
      upstream:
        - name: handler
          codefile:
            - name: character

- location: di
  codefile:
    - name: container
{% endif -%}
", ext = "txt")]
pub struct ManifestTmpl {
    pub lang: String,
    pub is_ddd: bool,
}
