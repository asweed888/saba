package model

import (
	"os"

	"github.com/asweed888/saba/utils"
)

type Declare struct {
    Lang string `yaml:"lang"`
    Arch string `yaml:"arch"`
    Spec []*DeclareSpec
    TacitSetting *TacitSetting
}

func createDirectory(workdir string) error {
    if err := os.MkdirAll(workdir, os.ModePerm); err != nil { return utils.Error(err) }

    return nil
}