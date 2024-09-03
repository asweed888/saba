use askama::Template;


#[derive(Template)]
#[template(source = "lang: {{ lang }}
{% if is_ddd -%}
arch: ddd
{% endif -%}
spec:
{% if !is_ddd -%}
- location: lib
  upstream:
    - name: traits
      codefile:
        - name: greet
{% else -%}
- location: domain
  upstream:
    - name: model
      upstream:
        - name: fish
          codefile:
            - name: entity

    - name: repository
      codefile:
        - name: fish


- location: infrastructure
  upstream:
    - name: repository
      codefile:
        - name: fish


- location: usecase
  codefile:
    - name: fish


- location: presentation
  upstream:
    - name: http
      upstream:
        - name: handler
          codefile:
            - name: aquarium

{% endif -%}
", ext = "txt")]
pub struct ManifestTemplate {
    pub lang: String,
    pub is_ddd: bool,
}