package datastore

import (
	"bytes"
	"html/template"

	"github.com/asweed888/saba/domain/model"
	"github.com/asweed888/saba/domain/repository"
	"github.com/asweed888/saba/utils"
)


type initialDeclareRepository struct {
	Lang string
	Arch string
}

func NewInitialDeclareRepository(lang string, arch string) repository.InitialDeclareRepository {
    return &initialDeclareRepository{lang, arch}
}


func (r *initialDeclareRepository) GetInitialDeclareFileContents() (*model.InitialDeclare, error) {
    var (
		re bytes.Buffer
		tmplStr string
	)

	if r.Arch == "ddd" {
		tmplStr = r.getDDDTemplate()
	} else {
		tmplStr = r.getPlainTemplate()
	}

    tmpl, err := template.New("saba").Parse(tmplStr)
    if err != nil {
        return nil, utils.Error(err)
    }

	err = tmpl.Execute(&re, map[string]interface{}{
		"Lang": r.Lang,
	}) // 置換して標準出力へ
	if err != nil {
		return nil, utils.Error(err)
	}


	return &model.InitialDeclare{
		FileContents: re.String(),
	}, nil
}


func (r *initialDeclareRepository) getDDDTemplate() string {
	return `lang: {{ .Lang }}
arch: ddd
spec:
- location: domain
  upstream:
    - name: model
	  codefile:
	    - name: example

	- name: repository
	  codefile:
	    - name: example


- location: infrastructure
  upstream:
    - name: datastore
	  codefile:
	    - name: example


- location: usecase
  codefile:
    - name: example


- location: presentation
  upstream:
    - name: http
	  upstream:
	    - name: handler
		  codefile:
		    - name: example

- location: di
  codefile:
    - name: mod`
}


func (r *initialDeclareRepository) getPlainTemplate() string {
	return `lang: {{ .Lang }}
spec:
- location: example
	`
}