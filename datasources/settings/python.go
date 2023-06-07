package settings

import "github.com/asweed888/saba/domain/model"

type pythonUtils struct {
	utils
}

var Python = &model.TacitSetting{
    Ext: "py",
    FileModeStr: "0644",
	TacitFileContents: func(s *model.TacitSetting, path string, fname string) string {
		ut := &pythonUtils{utils{s, path, fname}}

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
func (u *pythonUtils) DomainModelFileContents() string {
    tmpl := `class {{ .Fname | ToTitle }}:
    def __init__(self):
`


    return u.utils.CodeFileContents(tmpl)
}


// ドメインリポジトリのファイルの場合に書き出す内容
func (u *pythonUtils) DomainRepositoryFileContents() string {
    tmpl := `from abc import ABC, abstractmethod


class I{{ .Fname | ToTitle }}Repository(ABC):
    @abstractmethod
`

    return u.utils.CodeFileContents(tmpl)
}


// インフラストラクチャ層のファイルの場合に書き出す内容
func (u *pythonUtils) InfraFileContents() string {
    tmpl := `from domain.repository.{{ .Fname }} import I{{ .Fname | ToTitle }}Repository


class {{ .Fname | ToTitle }}Repository(I{{ .Fname | ToTitle }}Repository):
    def __init__(self):
`

    return u.utils.CodeFileContents(tmpl)
}


// ユースケース層のファイルの場合に書き出す内容
func (u *pythonUtils) UseCaseFileContents() string {
    tmpl := `from abc import ABC, abstractmethod


class I{{ .Fname | ToTitle }}UseCase(ABC):
    @abstractmethod


class {{ .Fname | ToTitle }}UseCase(I{{ .Fname | ToTitle }}UseCase):
    def __init__(self, repository):
        self.repository = repository
`

    return u.utils.CodeFileContents(tmpl)
}


// プレゼンテーション層のファイルの場合に書き出す内容
func (u *pythonUtils) PresentationFileContents() string {
    tmpl := `from abc import ABC, abstractmethod


class I{{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }}(ABC):
    @abstractmethod


class {{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }}(I{{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }}):
    def __init__(self, usecase):
        self.usecase = usecase
`

	if u.Fname == "app" || u.Fname == "mod" {
		return u.DefaultFileContents()
	} else {
		return u.utils.CodeFileContents(tmpl)
	}
}


// DIコンテナのファイルの場合に書き出す内容
func (u *pythonUtils) DiFileContents() string {
	tmpl := `from abc import ABC, abstractmethod
from infrastructure.datasrore. import
from usecase. import
from presentation. import


class IDIContainer(ABC):
    @abstractmethod


class DIContainer(IDIContainer):
    def __init__(self):
`

	return tmpl
}

// その他のファイルの場合に書き出す内容 (デフォルト)
func (u *pythonUtils) DefaultFileContents() string {
    tmpl := ``
    return tmpl
}