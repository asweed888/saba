package di

import (
	"github.com/asweed888/saba/domain/repository"
	"github.com/asweed888/saba/infrastructure/datastore"
	"github.com/asweed888/saba/presentation/command/handler"
	"github.com/asweed888/saba/usecase"
)

type DiContainer interface {
	// 宣言ファイル
    NewDeclareRepository() repository.DeclareRepository
    NewDeclareUseCase() usecase.DeclareUseCase

	// saba newコマンド用宣言ファイル
	NewInitialDeclareRepository() repository.InitialDeclareRepository
	NewInitialDeclareUseCase() usecase.InitialDeclareUseCase

	// コマンドハンドラー
	NewUpCmdHandler() handler.UpCmdHandler
	NewNewCmdHandler() handler.NewCmdHandler
	NewAppHandler() handler.AppHandler
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
	handler.UpCmdHandler
	handler.NewCmdHandler
}

func (c *diContainer) NewUpCmdHandler() handler.UpCmdHandler {
	return handler.NewUpCmdHandler(c.NewDeclareUseCase())
}

func (c *diContainer) NewNewCmdHandler() handler.NewCmdHandler {
	return handler.NewNewCmdHandler(c.NewInitialDeclareUseCase())
}

func (c *diContainer) NewAppHandler() handler.AppHandler {
	subCmd := &subCommand{}
	subCmd.UpCmdHandler = c.NewUpCmdHandler()
	subCmd.NewCmdHandler = c.NewNewCmdHandler()
	return subCmd
}