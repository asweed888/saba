use domain::model::manifest::entity::Manifest;
use domain::repository::manifest::ManifestRepository;
use anyhow::Result;
use askama::Template;


pub struct ManifestUseCase<R>
where
    R: ManifestRepository,
{
    pub repository: R,
}

impl<R> ManifestUseCase<R>
where
    R: ManifestRepository,
{
    pub fn new(repository: R) -> Self {
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
- location: character
  codefile:
    - name: entity
{% else -%}
- location: domain
  upstream:
    - name: model
      upstream:
        - name: character
          codefile:
            - name: entity

    - name: repository
      codefile:
        - name: character


- location: infrastructure
  upstream:
    - name: repository
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