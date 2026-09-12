// zettel-merken is your daily review helper!
package main

import (
	"errors"
	"fmt"
	"log/slog"
	"os"

	"github.com/p-tupe/zettel-merken/internal/config"
	"github.com/p-tupe/zettel-merken/internal/initialize"
)

const helpText = `zettel-merken is your daily review helper!

Usage:

  zettel-merken init <notes_dir>    Initial setup
  zettel-merken config              Opens config file
  zettel-merken help                Show this help
  zettel-merken run                 Daily review run`

func main() {
	if len(os.Args) <= 1 {
		slog.Error("no valid option supplied")
		os.Exit(1)
	}

	if err := dispatch(os.Args[1], os.Args); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}

}

func dispatch(command string, all []string) error {
	switch command {
	case "init":
		if len(all) < 3 {
			return errors.New("need a notes dir with init")
		}

		if err := initialize.Setup(all[2]); err != nil {
			return fmt.Errorf("could not initialize, %w", err)
		}

	case "config":
		if err := config.Edit(); err != nil {
			return fmt.Errorf("cannot opening config, %w", err)
		}

	case "run":
		if err := dailyRun(); err != nil {
			return fmt.Errorf("daily run failed, %w", err)
		}

	case "help":
		fmt.Println(helpText)

	default:
		return fmt.Errorf("error: unknown option %s", os.Args[1])
	}

	return nil
}
