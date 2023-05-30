package main

import (
	"github.com/spf13/cobra"
)

var Version string


func main(){

    a := app.NewApp("./saba.yml")
    subcmd := a.NewSubCommand()


    cli := &cobra.Command{
        Use: "saba",
        Short: "This is a very simple declarative development framework.",
        Version: Version,
    }

    cli.AddCommand(subcmd.BuildCmd())
    cli.Execute()
}
