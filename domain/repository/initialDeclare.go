package repository

import "github.com/asweed888/saba/domain/model"

type InitialDeclareRepository interface {
	GetInitialDeclareFileContents() (*model.InitialDeclare, error)
}