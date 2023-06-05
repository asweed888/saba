package settings

import "github.com/asweed888/saba/domain/model"



var Bash = &model.TacitSetting{
    FileModeStr: "0755",
    TacitFileContents: func(c *model.TacitSetting, path string, fname string) string {
        return "#!/bin/bash"
    },
}