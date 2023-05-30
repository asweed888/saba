package model

import (
	"io/ioutil"
	"os"
)

type DeclareCodeFile struct {
    Name string `yaml:"name"`
    Description string `yaml:"description"`
	TacitSetting *TacitSetting
}

func (d *DeclareCodeFile) CreateCodeFile(workdir string) error {
    path := d.TacitSetting.CodeFileFullPath(workdir, d.Name)
    ifc := d.TacitSetting.FileContents(workdir, d.Name)

    if _, err := os.Stat(path); err != nil {
        err = ioutil.WriteFile(path, []byte(ifc), d.TacitSetting.FileMode())
        if err != nil { return err }

    }
    return nil
}


func (d *DeclareCodeFile) SetTacitSetting(conf *TacitSetting) {
    d.TacitSetting = conf
}