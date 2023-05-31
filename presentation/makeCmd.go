package presentation

import (
	"log"

	"github.com/asweed888/saba/usecase"
	"github.com/asweed888/saba/utils"
	"github.com/spf13/cobra"
)

type MakeCmdPresentation interface {
	MakeCmd() *cobra.Command
}

type makeCmdPresentation struct {
	usecase.DeclareUseCase
}

func NewMakeCmdPresentation(u usecase.DeclareUseCase) MakeCmdPresentation {
    return &makeCmdPresentation{u}
}

func (p *makeCmdPresentation) MakeCmd() *cobra.Command {
	return &cobra.Command{
		Use: "make",
		Aliases: []string{"mk", "m"},
		Short: "Automatically generate directories and code files based on the structure described in saba.yml.",
		RunE: func(cmd *cobra.Command, args []string) error {
			err := p.DeclareUseCase.GenerateArch()
			if err != nil {
				return utils.Error(err)
			}

            log.Println("generate of clerk has been completed.")
			return nil
		},
	}
}