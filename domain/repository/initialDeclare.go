package repository

import "github.com/asweed888/saba/domain/model"

type InitialDeclareRepository interface {
	GetInitialDeclareFileContents(lang string, isDDD string) (*model.InitialDeclare, error)
}