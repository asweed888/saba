package usecase

import (
	"github.com/asweed888/saba/domain/model"
	"github.com/asweed888/saba/domain/repository"
	"github.com/asweed888/saba/utils"
)


type DeclareUseCase interface {
	GenerateArch() error
}

type declareUseCase struct {
    repository.DeclareRepository
}

func NewDeclareUseCase(r repository.DeclareRepository) DeclareUseCase {
    return &declareUseCase{r}
}

func (u *declareUseCase) GenerateArch() error {
    decl, _ := u.DeclareRepository.Load()
    conf := decl.TacitSetting

    for _, s := range decl.Spec {
        s.SetTacitSetting(conf)
        workdir := s.ChangeDirectory(".")
        if err := s.CreateDirectory(workdir); err != nil { return utils.Error(err) }

        if len(s.Upstream) != 0 {
            if err := generateUpstream(workdir, s.Upstream, conf); err != nil { return utils.Error(err) }
        }

        if len(s.CodeFile) != 0 {
            if err := generateCodeFile(workdir, s.CodeFile, conf); err != nil { return utils.Error(err) }
        }
    }

    return nil
}

func generateUpstream(prevWorkDir string, upstream []*model.DeclareUpstream, conf *model.TacitSetting) error {

    for _, ups := range upstream {
        ups.SetTacitSetting(conf)
        workdir := ups.ChangeDirectory(prevWorkDir)
        if err := ups.CreateDirectory(workdir); err != nil { return utils.Error(err) }

        if len(ups.Upstream) != 0 {
            if err := generateUpstream(workdir, ups.Upstream, conf); err != nil { return utils.Error(err) }
        }

        if len(ups.CodeFile) != 0 {
            if err := generateCodeFile(workdir, ups.CodeFile, conf); err != nil { return utils.Error(err) }
        }
    }
    return nil
}

func generateCodeFile(workdir string, codeFile []*model.DeclareCodeFile, conf *model.TacitSetting) error {

    for _, c := range codeFile {
        c.SetTacitSetting(conf)
        c.CreateCodeFile(workdir)
    }
    return nil
}