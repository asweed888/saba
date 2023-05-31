package di

import (
	"github.com/asweed888/saba/domain/repository"
	"github.com/asweed888/saba/infrastructure/datastore"
	"github.com/asweed888/saba/presentation"
	"github.com/asweed888/saba/usecase"
)

type DiContainer interface {
    NewDeclareRepository() repository.DeclareRepository
    NewDeclareUseCase() usecase.DeclareUseCase
	NewMakeCmd() presentation.MakeCmdPresentation
	NewSubCmd() presentation.SubCmdPresentation
}

type diContainer struct {
	DeclareFile string
}

func NewDiContainer(declareFile string) DiContainer {
	return &diContainer{declareFile}
}


func (c *diContainer) NewDeclareRepository() repository.DeclareRepository {
	return datastore.NewDeclareRepository(c.DeclareFile)
}

func (c *diContainer) NewDeclareUseCase() usecase.DeclareUseCase {
	return usecase.NewDeclareUseCase(c.NewDeclareRepository())
}


type subCommand struct {
	presentation.MakeCmdPresentation
}

func (c *diContainer) NewMakeCmd() presentation.MakeCmdPresentation {
	return presentation.NewMakeCmdPresentation(c.NewDeclareUseCase())
}

func (c *diContainer)NewSubCmd() presentation.SubCmdPresentation {
	subCmd := &subCommand{}
	subCmd.MakeCmdPresentation = c.NewMakeCmd()
	return subCmd
}