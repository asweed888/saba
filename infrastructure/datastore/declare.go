package datastore

import (
    "errors"
    "io/ioutil"

    "github.com/asweed888/saba/datasources/settings"
    "github.com/asweed888/saba/domain/model"
    "github.com/asweed888/saba/domain/repository"
    "github.com/asweed888/saba/utils"
    "gopkg.in/yaml.v2"
)


type declareRepository struct {
    DeclareFile string
}

func NewDeclareRepository(declareFile string) repository.DeclareRepository {
    return &declareRepository{declareFile}
}

func (r *declareRepository) Load() (*model.Declare, error) {


    // 宣言ファイルを開く
    f, err := ioutil.ReadFile(r.DeclareFile)
    if err != nil {
        return nil, utils.Error(err)
    }

    // 読み込んだ宣言ファイルをDeclare構造体に注入
    var declare *model.Declare
    err = yaml.Unmarshal(f, &declare)
    if err != nil {
        return nil, utils.Error(err)
    }

    // 暗黙の設定を読み込む
    setting, err := loadTacitSetting(declare.Lang)
    if err != nil {
        return nil, utils.Error(err)
    }

    setting.Lang = declare.Lang
    setting.Arch = declare.Arch
    declare.TacitSetting = setting

    return declare, nil
}


func loadTacitSetting(lang string) (*model.TacitSetting, error) {
    switch lang {
    case "go":
        return settings.Golang, nil
    case "rust":
        return settings.Rust, nil
    case "v":
        return settings.Vlang, nil
    case "python":
        return settings.Python, nil
    case "bash":
        return settings.Bash, nil
    case "javascript":
        return settings.Javascript, nil
    default:
        err := errors.New("Invalid programming language specified")
        return nil, utils.Error(err)
    }
}