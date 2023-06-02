package presentation

import (
	"github.com/asweed888/saba/usecase"
	"github.com/spf13/cobra"
)

type NewCmdPresentation interface {
	NewCmd() *cobra.Command
}

type newCmdPresentation struct {
    usecase.InitialDeclareUseCase
}

func NewNewCmdPresentation(u usecase.InitialDeclareUseCase) NewCmdPresentation {
    return &newCmdPresentation{u}
}


func (p *newCmdPresentation) NewCmd() *cobra.Command {
	return &cobra.Command{
		Use: "new",
		Short: "Generate saba.yml.",
		RunE: func(cmd *cobra.Command, args []string) error {

		},
	}
}