package presentation

type MakeCmdPresentation interface {}

type makeCmdPresentation struct {
    usecase.MakeCmdUseCase
}

func NewMakeCmdPresentation(u usecase.MakeCmdUseCase) MakeCmdPresentation {
    return &makeCmdPresentation{u}
}