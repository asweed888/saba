package datastore

import (
	"errors"
	"io/ioutil"

	"github.com/asweed888/saba/datasources/settings"
	"github.com/asweed888/saba/domain/model"
	"github.com/asweed888/saba/domain/repository"
	"golang.org/x/xerrors"
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
		return nil, xerrors.Errorf("%v", err)
	}

	// 読み込んだ宣言ファイルをDeclare構造体に注入
	var declare *model.Declare
	err = yaml.Unmarshal(f, &declare)
	if err != nil {
		return nil, xerrors.Errorf("%v", err)
	}

	// 暗黙の設定を読み込む

}


func loadTacitSetting(lang string) (*model.TacitSetting, error) {
	switch lang {
	case "go":
		return settings.Golang, nil
	default:
		return nil, errors.New("Invalid programming language specified")

	}
}