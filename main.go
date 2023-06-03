package main

import (
	"github.com/asweed888/saba/di"
	"github.com/spf13/cobra"
)

var Version string


func main(){

    c := di.NewDiContainer("./saba.yml")
    subcmd := c.NewSubCmd()


    cli := &cobra.Command{
        Use: "saba",
        Short: "This is a very simple declarative development framework.",
        Version: Version,
    }

    cli.AddCommand(
		subcmd.UpCmd(),
		subcmd.NewCmd(),
	)
    cli.Execute()
}
