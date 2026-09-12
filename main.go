// zettel-merken is your daily review notifier for studing.
//
// Usage:
//
//	$ zettel-merken init <notes_dir> # Initial Setup
//	$ zettel-merken config # Opens config file
//	$ zettel-merken help # Daily Review run
//	$ zettel-merken # Daily Review run
package main

import (
	"fmt"
	"log/slog"
	"os"

	"github.com/p-tupe/zettel-merken/internal/config"
	"github.com/p-tupe/zettel-merken/internal/help"
	"github.com/p-tupe/zettel-merken/internal/initialize"
	"github.com/p-tupe/zettel-merken/internal/run"
)

func main() {
	if len(os.Args) <= 1 {
		slog.Error("Error: no valid option supplied. See `zettel-merken help` for usage.")
		return
	}

	switch os.Args[1] {
	case "init":
		if err := initialize.Setup(); err != nil {
			slog.Error("Error while initilizing", "err", err.Error())
		}

	case "config":
		if err := config.Edit(); err != nil {
			slog.Error("Error while opening config", "err", err.Error())
		}

	case "help":
		help.Show()

	case "run":
		if err := run.Run(); err != nil {
			slog.Error("Error during daily run", "err", err)
		}

	default:
		slog.Error(fmt.Sprintf("Error parsing option %s. Must be one of [init | run | config | help]", os.Args[1]))
		return
	}
}
