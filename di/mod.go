package di

import (
	"github.com/asweed888/saba/domain/repository"
	"github.com/asweed888/saba/infrastructure/datastore"
	"github.com/asweed888/saba/presentation"
	"github.com/asweed888/saba/usecase"
)

type DiContainer interface {
	// 宣言ファイル
    NewDeclareRepository() repository.DeclareRepository
    NewDeclareUseCase() usecase.DeclareUseCase

	// saba newコマンド用宣言ファイル
	NewInitialDeclareRepository() repository.InitialDeclareRepository
	NewInitialDeclareUseCase() usecase.InitialDeclareUseCase

	// コマンド
	NewUpCmd() presentation.UpCmdPresentation
	NewNewCmd() presentation.NewCmdPresentation
	NewSubCmd() presentation.SubCmdPresentation
}

type diContainer struct {
	DeclareFile string
}

func NewDiContainer(declareFile string) DiContainer {
	return &diContainer{declareFile}
}

// 宣言ファイル
func (c *diContainer) NewDeclareRepository() repository.DeclareRepository {
	return datastore.NewDeclareRepository(c.DeclareFile)
}

func (c *diContainer) NewDeclareUseCase() usecase.DeclareUseCase {
	return usecase.NewDeclareUseCase(c.NewDeclareRepository())
}


// saba newで生成されるsaba.ymlの内容
func (c *diContainer) NewInitialDeclareRepository() repository.InitialDeclareRepository {
	return datastore.NewInitialDeclareRepository()
}

func (c *diContainer) NewInitialDeclareUseCase() usecase.InitialDeclareUseCase {
	return usecase.NewInitialDeclareUseCase(c.NewInitialDeclareRepository())
}


// コマンド
type subCommand struct {
	presentation.UpCmdPresentation
	presentation.NewCmdPresentation
}

func (c *diContainer) NewUpCmd() presentation.UpCmdPresentation {
	return presentation.NewUpCmdPresentation(c.NewDeclareUseCase())
}

func (c *diContainer) NewNewCmd() presentation.NewCmdPresentation {
	return presentation.NewNewCmdPresentation(c.NewInitialDeclareUseCase())
}

func (c *diContainer)NewSubCmd() presentation.SubCmdPresentation {
	subCmd := &subCommand{}
	subCmd.UpCmdPresentation = c.NewUpCmd()
	subCmd.NewCmdPresentation = c.NewNewCmd()
	return subCmd
}