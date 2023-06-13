package handler

import (
	"log"

	"github.com/asweed888/saba/usecase"
	"github.com/asweed888/saba/utils"
	"github.com/spf13/cobra"
)

type UpCmdHandler interface {
	UpCmd() *cobra.Command
}

type upCmdHandler struct {
	usecase.DeclareUseCase
}

func NewUpCmdHandler(u usecase.DeclareUseCase) UpCmdHandler {
	return &upCmdHandler{u}
}

func (h *upCmdHandler) UpCmd() *cobra.Command {
	return &cobra.Command{
		Use:   "up",
		Short: "Automatically generate directories and code files based on the structure described in saba.yml.",
		RunE: func(cmd *cobra.Command, args []string) error {
			err := h.DeclareUseCase.GenerateArch()
			if err != nil {
				return utils.Error(err)
			}

			err = h.DeclareUseCase.SubAction()
			if err != nil {
				utils.Error(err)
			}

			log.Println("generate of saba has been completed.")
			return nil
		},
	}
}