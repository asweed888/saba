package repository

import "github.com/asweed888/saba/domain/model"

type DeclareRepository interface {
    Load() (*model.Declare, error)
}