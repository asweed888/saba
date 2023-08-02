package settings

import "github.com/asweed888/saba/domain/model"

type tsUtils struct {
    utils
}

var Typescript = &model.TacitSetting{
    Ext: "ts",
    FileModeStr: "0644",
    TacitFileContents: func(s *model.TacitSetting, path string, fname string) string {
        ut := &tsUtils{utils{s, path, fname}}

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
func (u *tsUtils) DomainModelFileContents() string {
    tmpl := `export class {{ .Fname | ToTitle }} {
  constructor() {}
}
`

    return u.utils.CodeFileContents(tmpl)
}


// ドメインリポジトリのファイルの場合に書き出す内容
func (u *tsUtils) DomainRepositoryFileContents() string {
    tmpl := `import { {{ .Fname | ToTitle }} } from '../model/{{ .Fname }}'

export interface I{{ .Fname | ToTitle }}Repository {}
`

    return u.utils.CodeFileContents(tmpl)
}


// インフラストラクチャ層のファイルの場合に書き出す内容
func (u *tsUtils) InfraFileContents() string {
    tmpl := `import { {{ .Fname | ToTitle }} } from '../../domain/model/{{ .Fname }}'
import { I{{ .Fname | ToTitle }}Repository } from '../../domain/repository/{{ .Fname }}'

export class {{ .Fname | ToTitle }}Repository implements I{{ .Fname | ToTitle }}Repository {
  constructor() {}
}
`

    return u.utils.CodeFileContents(tmpl)
}


// ユースケース層のファイルの場合に書き出す内容
func (u *tsUtils) UseCaseFileContents() string {
    tmpl := `import { {{ .Fname | ToTitle }} } from '../domain/model/{{ .Fname }}'
import { I{{ .Fname | ToTitle }}Repository } from '../domain/repository/{{ .Fname }}'

export interface I{{ .Fname | ToTitle }}UseCase {}

export class {{ .Fname | ToTitle }}UseCase implements I{{ .Fname | ToTitle }}UseCase {
  repository: I{{ .Fname | ToTitle }}Repository

  constructor(repository: I{{ .Fname | ToTitle }}Repository) {
    this.repository = repository
  }
}
`

    return u.utils.CodeFileContents(tmpl)
}


// プレゼンテーション層のファイルの場合に書き出す内容
func (u *tsUtils) PresentationFileContents() string {
    tmpl := `import { {{ .Fname | ToTitle }} } from '../../../domain/model/{{ .Fname }}'
import { I{{ .Fname | ToTitle }}UseCase } from '../../../usecase/{{ .Fname }}'

export interface I{{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }} {}

export class {{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }} implements I{{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }} {
  usecase: I{{ .Fname | ToTitle }}UseCase

  constructor(usecase: I{{ .Fname | ToTitle }}UseCase) {
    this.usecase = usecase
  }
}
`

    if u.Fname == "app" || u.Fname == "mod" || u.Setting.IsNoExtensionGroup(u.Fname) {
        return u.DefaultFileContents()
    } else {
        return u.utils.CodeFileContents(tmpl)
    }
}


// DIコンテナのファイルの場合に書き出す内容
func (u *tsUtils) DiFileContents() string {
    tmpl := `import {  } from '../domain/repository'
import {  } from '../infrastructure/datastore'
import {  } from '../presentation'

interface IDIContainer {}

export class DIContainer implements IDIContainer {
  constructor() {}
}
`

    return tmpl
}

// その他のファイルの場合に書き出す内容 (デフォルト)
func (u *tsUtils) DefaultFileContents() string {
    tmpl := ``
    return tmpl
}