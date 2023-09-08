package settings

import (
    "bytes"
    "html/template"
    "strings"

    "github.com/asweed888/saba/domain/model"
)

type Utils interface {
    Pkgname() string
    IsDDD() bool
    IsDomainModel() bool
    IsDomainRepository() bool
    IsInfra() bool
    IsUseCase() bool
    IsPresentation() bool
    CodeFileContents(tmplStr string) string
    codeFileContentsValues() map[string]interface{}
}

type utils struct {
    Setting *model.TacitSetting
    Path string
    Fname string
}


func (u *utils) Pkgname() string {
    idx := strings.LastIndexByte(u.Path, '/') + 1
    return u.Path[idx:]
}


func (u *utils) IsDDD() bool {
    return  u.Setting.Arch == "ddd"
}

func (u *utils) IsDomainModel() bool {
    return u.IsDDD() && strings.Contains(u.Path, "/domain/model")
}

func (u *utils) IsDomainRepository() bool {
    return u.IsDDD() && strings.Contains(u.Path, "/domain/repository")
}

func (u *utils) IsDomainService() bool {
    return u.IsDDD() && strings.Contains(u.Path, "/domain/service")
}

func (u *utils) IsInfra() bool {
    return u.IsDDD() && strings.Contains(u.Path, "/infra")
}

func (u *utils) IsUseCase() bool {
    return u.IsDDD() && strings.Contains(u.Path, "/usecase")
}

func (u *utils) IsPresentation() bool {
    return u.IsDDD() && strings.Contains(u.Path, "/presentation")
}

func (u *utils) IsDi() bool {
    return u.IsDDD() && u.Pkgname() == "di"
}


func toTitleCase(input string) string {
    if strings.Contains(input, "_") {
        parts := strings.Split(input, "_") // アンダースコアで文字列を分割
        var result []string

        for _, part := range parts {
            // 各パート（単語）をTitle Caseに変換
            titlePart := strings.Title(part)
            result = append(result, titlePart)
        }

        return strings.Join(result, "")
    }

    // アンダースコアが含まれていない場合はそのままTitle Caseに変換
    return strings.Title(input)
}

func (u *utils) CodeFileContents(tmplStr string) string {
    var re bytes.Buffer
    funcMap := template.FuncMap{
        "ToTitle": toTitleCase,
    }

    tmpl, err := template.New("saba").Funcs(funcMap).Parse(tmplStr)
    if err != nil {
        return ""
    }

    err = tmpl.Execute(&re, u.codeFileContentsValues()) // 置換して標準出力へ
    if err != nil {
        return ""
    }


    return re.String()

}

func (u *utils) codeFileContentsValues() map[string]interface{} {
    return map[string]interface{}{
        "Pkgname": u.Pkgname(),
        "Fname": u.Fname,
    }
}