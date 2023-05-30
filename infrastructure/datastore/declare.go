package datastore

type declareRepository struct {}

func NewDeclareRepository() repository.DeclareRepository {
    return &declareRepository{}
}