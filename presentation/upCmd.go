package presentation

import (
	"log"

	"github.com/asweed888/saba/usecase"
	"github.com/asweed888/saba/utils"
	"github.com/spf13/cobra"
)

type UpCmdPresentation interface {
	UpCmd() *cobra.Command
}

type upCmdPresentation struct {
    usecase.DeclareUseCase
}

func NewUpCmdPresentation(u usecase.DeclareUseCase) UpCmdPresentation {
    return &upCmdPresentation{u}
}


func (p *upCmdPresentation) UpCmd() *cobra.Command {
	return &cobra.Command{
		Use: "up",
		Short: "Automatically generate directories and code files based on the structure described in saba.yml.",
		RunE: func(cmd *cobra.Command, args []string) error {
			err := p.DeclareUseCase.GenerateArch()
			if err != nil {
				return utils.Error(err)
			}

            log.Println("generate of saba has been completed.")
			return nil
		},
	}
}