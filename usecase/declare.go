package usecase

type DeclareUseCase interface {}

type declareUseCase struct {
    repository.DeclareRepository
}

func NewDeclareUseCase(r repository.DeclareRepository) DeclareUseCase {
    return &declareUseCase{r}
}