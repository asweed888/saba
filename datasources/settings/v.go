package settings

import "github.com/asweed888/saba/domain/model"

type vUtils struct {
	utils
}

var Vlang = &model.TacitSetting{
    Ext: "v",
    FileModeStr: "0644",
	TacitFileContents: func(s *model.TacitSetting, path string, fname string) string {
		ut := &vUtils{utils{s, path, fname}}

        if ut.IsDomainModel() {
            return ut.DomainModelFileContents()
        } else if ut.IsDomainRepository() {
            return ut.DomainRepositoryFileContents()
        } else if ut.IsInfra() {
            return ut.InfraFileContents()
        } else if ut.IsUseCase() {
            return ut.UseCaseFileContents()
        } else if ut.IsPresentation() {
            return ut.PresentationFileContents()
        } else if ut.IsDi() {
            return ut.DiFileContents()
        } else {
            return ut.DefaultFileContents()
        }
	},
}


// ドメインモデルのファイルの場合に書き出す内容
func (u *vUtils) DomainModelFileContents() string {
    tmpl := `module {{ .Pkgname }}

pub struct {{ .Fname | ToTitle }} {}
`


    return u.utils.CodeFileContents(tmpl)
}


// ドメインリポジトリのファイルの場合に書き出す内容
func (u *vUtils) DomainRepositoryFileContents() string {
    tmpl := `module {{ .Pkgname }}

import domain.model

pub interface I{{ .Fname | ToTitle }}Repository {}
`

    return u.utils.CodeFileContents(tmpl)
}


// インフラストラクチャ層のファイルの場合に書き出す内容
func (u *vUtils) InfraFileContents() string {
    tmpl := `module {{ .Pkgname }}

import domain.model
import domain.repository

struct {{ .Fname | ToTitle }}Repository {}

pub fn new_{{ .Fname }}_repository() repository.I{{ .Fname | ToTitle }}Repository {
    return {{ .Fname | ToTitle }}Repository{}
}
`

    return u.utils.CodeFileContents(tmpl)
}


// ユースケース層のファイルの場合に書き出す内容
func (u *vUtils) UseCaseFileContents() string {
    tmpl := `module {{ .Pkgname }}

import domain.model
import domain.repository

pub interface I{{ .Fname | ToTitle }}UseCase {}

struct {{ .Fname | ToTitle }}UseCase {
    repository.I{{ .Fname | ToTitle }}Repository
}

pub fn new_{{ .Fname }}_usecase(r repository.I{{ .Fname | ToTitle }}Repository) I{{ .Fname | ToTitle }}UseCase {
    return {{ .Fname | ToTitle }}UseCase{r}
}
`

    return u.utils.CodeFileContents(tmpl)
}


// プレゼンテーション層のファイルの場合に書き出す内容
func (u *vUtils) PresentationFileContents() string {
    tmpl := `module {{ .Pkgname }}

import domain.model
import usecase

pub interface I{{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }} {}

struct {{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }} {
    usecase.I{{ .Fname | ToTitle }}UseCase
}

pub fn new_{{ .Fname }}_{{ .Pkgname }}(u usecase.I{{ .Fname | ToTitle }}UseCase) I{{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }} {
    return {{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }}{u}
}
`

	if u.Fname == "app" || u.Fname == "mod" {
		return u.DefaultFileContents()
	} else {
		return u.utils.CodeFileContents(tmpl)
	}
}


// DIコンテナのファイルの場合に書き出す内容
func (u *vUtils) DiFileContents() string {
	tmpl := `module di

import domain.model
import domain.repository
import infrastructure
import usecase
import presentation

pub interface IDiContainer {}

struct DiContainer{}

pub fn new_di_container() IDiContainer {
	return DiContainer{}
}
`

	return tmpl
}

// その他のファイルの場合に書き出す内容 (デフォルト)
func (u *vUtils) DefaultFileContents() string {
    tmpl := `module {{ .Pkgname }}`
    return u.utils.CodeFileContents(tmpl)
}