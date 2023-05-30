package presentation

type SubCmdPresentation interface {}

type subCmdPresentation struct {
    usecase.SubCmdUseCase
}

func NewSubCmdPresentation(u usecase.SubCmdUseCase) SubCmdPresentation {
    return &subCmdPresentation{u}
}