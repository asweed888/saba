package model

import "fmt"

type DeclareSpec struct {
    Location string `yaml:"location"`
    Upstream []*DeclareUpstream
    CodeFile []*DeclareCodeFile
    TacitSetting *TacitSetting
}

func (d *DeclareSpec) CreateDirectory(workdir string) error {
    return createDirectory(workdir)
}

func (d *DeclareSpec) ChangeDirectory(prevWorkDir string) string {
    return fmt.Sprintf("%s/%s", prevWorkDir, d.Location)
}

func (d *DeclareSpec) SetTacitSetting(conf *TacitSetting) {
    d.TacitSetting = conf
}