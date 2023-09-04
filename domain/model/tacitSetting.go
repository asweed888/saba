package model

import (
    "fmt"
    "io/fs"
    "os"
    "strconv"
    "strings"
)

type TacitSetting struct {
    Lang              string
    Arch              string
    DefaultRootPath   string
    Ext               string
    FileModeStr       string
    TacitFileContents func(conf *TacitSetting, path string, fname string) string
    TacitSubAction    func(decl *Declare) error
}

func (s *TacitSetting) RootPath(decl *Declare) string {
    if decl.Root != "" {
        return fmt.Sprintf("./%s", decl.Root)
    } else if decl.Root == "" && s.DefaultRootPath != "" {
        return fmt.Sprintf("./%s", s.DefaultRootPath)
    } else {
        return "."
    }
}

func (s *TacitSetting) FileMode() fs.FileMode {
    perm32, _ := strconv.ParseUint(s.FileModeStr, 8, 32)
    return os.FileMode(perm32)
}

func (s *TacitSetting) CodeFileFullPath(path string, fname string) string {
    if s.Ext == "" || s.IsNoExtensionGroup(fname) {
        return fmt.Sprintf("%s/%s", path, fname)
    } else {
        return fmt.Sprintf("%s/%s.%s", path, fname, s.Ext)
    }
}

func (s *TacitSetting) FileContents(path string, fname string) string {
    if s.TacitFileContents == nil {
        return ""
    }
    return s.TacitFileContents(s, path, fname)
}

func (s *TacitSetting) SubAction(decl *Declare) error {

    if s.TacitSubAction == nil {
        return nil
    }
    return s.TacitSubAction(decl)
}

func (s *TacitSetting) IsNoExtensionGroup(fname string) bool {
    return strings.Contains(fname, ".svelte") || strings.Contains(fname, ".tsx") || strings.Contains(fname, ".vue")
}