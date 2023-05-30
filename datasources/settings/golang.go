package settings

import "github.com/asweed888/saba/domain/model"

type golangUtils struct {
	utils
}

var Golang = &model.TacitSetting{
    Ext: "go",
    FileModeStr: "0644",
    TacitFileContents: func(c *model.TacitSetting, path string, fname string) string {

        ut := &golangUtils{utils{c, path, fname}}

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
func (u *golangUtils) DomainModelFileContents() string {
    tmpl := `package {{ .Pkgname }}

type {{ .Fname | ToTitle }} struct {}`


    return u.utils.CodeFileContents(tmpl)
}


// ドメインリポジトリのファイルの場合に書き出す内容
func (u *golangUtils) DomainRepositoryFileContents() string {
    tmpl := `package {{ .Pkgname }}

type {{ .Fname | ToTitle }}Repository interface {}`

    return u.utils.CodeFileContents(tmpl)
}


// インフラストラクチャ層のファイルの場合に書き出す内容
func (u *golangUtils) InfraFileContents() string {
    tmpl := `package {{ .Pkgname }}

type {{ .Fname }}Repository struct {}

func New{{ .Fname | ToTitle }}Repository() repository.{{ .Fname | ToTitle }}Repository {
    return &{{ .Fname }}Repository{}
}`

    return u.utils.CodeFileContents(tmpl)
}


// ユースケース層のファイルの場合に書き出す内容
func (u *golangUtils) UseCaseFileContents() string {
    tmpl := `package {{ .Pkgname }}

type {{ .Fname | ToTitle }}UseCase interface {}

type {{ .Fname }}UseCase struct {
    repository.{{ .Fname | ToTitle }}Repository
}

func New{{ .Fname | ToTitle }}UseCase(r repository.{{ .Fname | ToTitle }}Repository) {{ .Fname | ToTitle }}UseCase {
    return &{{ .Fname }}UseCase{r}
}`

    return u.utils.CodeFileContents(tmpl)
}


// プレゼンテーション層のファイルの場合に書き出す内容
func (u *golangUtils) PresentationFileContents() string {
    tmpl := `package {{ .Pkgname }}

type {{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }} interface {}

type {{ .Fname }}{{ .Pkgname | ToTitle }} struct {
    usecase.{{ .Fname | ToTitle }}UseCase
}

func New{{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }}(u usecase.{{ .Fname | ToTitle }}UseCase) {{ .Fname | ToTitle }}{{ .Pkgname | ToTitle }} {
    return &{{ .Fname }}{{ .Pkgname | ToTitle }}{u}
}`

	if u.Fname == "app" || u.Fname == "mod" {
		return u.DefaultFileContents()
	} else {
		return u.utils.CodeFileContents(tmpl)
	}
}


// DIコンテナのファイルの場合に書き出す内容
func (u *golangUtils) DiFileContents() string {
	tmpl := `package di

type DiContainer interface {}

type diContainer struct {}

func NewDiContainer() DiContainer {
	return &diContainer{}
}`

	return tmpl
}

// その他のファイルの場合に書き出す内容 (デフォルト)
func (u *golangUtils) DefaultFileContents() string {
    tmpl := `package {{ .Pkgname }}`
    return u.utils.CodeFileContents(tmpl)
}