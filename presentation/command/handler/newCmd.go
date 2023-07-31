package handler

import (
    "fmt"

    "github.com/asweed888/saba/usecase"
    "github.com/asweed888/saba/utils"
    "github.com/manifoldco/promptui"
    "github.com/spf13/cobra"
)

type NewCmdHandler interface {
    NewCmd() *cobra.Command
}

type newCmdHandler struct {
    usecase.InitialDeclareUseCase
}

func NewNewCmdHandler(u usecase.InitialDeclareUseCase) NewCmdHandler {
    return &newCmdHandler{u}
}

func (h *newCmdHandler) NewCmd() *cobra.Command {
    return &cobra.Command{
        Use: "new",
        Short: "Generate saba.yml.",
        RunE: func(cmd *cobra.Command, args []string) error {

            prompt := promptui.Select{
                Label: "Please select a programming language.",
                Items: []string{
                    "rust",
                    "javascript",
                    "go",
                    "python",
                    "bash",
                    "v",
                },
                Size:      6,
            }

            _, lang, err := prompt.Run()
            if err != nil {
                return utils.Error(err)
            }

            var isDDD string
            if lang == "bash" {
                isDDD = "No"
            } else {
                prompt = promptui.Select{
                    Label: "Do you want to develop applications with ddd (onion architecture)?",
                    Items: []string{
                        "No",
                        "Yes",
                    },
                }

                _, is_ddd, err := prompt.Run()
                if err != nil {
                    return utils.Error(err)
                }
                isDDD = is_ddd
            }

            fmt.Printf("Programming language to use → %q\n", lang)
            fmt.Printf("Using ddd(onion architecture) → %q\n", isDDD)

            err = h.GenerateInitialDeclareFile(lang, isDDD)
            if err != nil {
                return utils.Error(err)
            }

            return nil
        },
    }
}