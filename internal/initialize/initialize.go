package initialize

import (
	"fmt"

	"github.com/p-tupe/zettel-merken/internal/config"
	"github.com/p-tupe/zettel-merken/internal/store"
	"github.com/p-tupe/zettel-merken/internal/utils"
)

func Setup(notesDir string) error {
	if !utils.IsReadableDir(notesDir) {
		return fmt.Errorf("cannot read notes directory %s", notesDir)
	}

	cfgPath, err := config.Path()
	if err != nil {
		return err
	}
	if utils.IsFileReadable(cfgPath) {
		return fmt.Errorf("config already exists, use `config` command to edit")
	}

	if err := config.Setup(notesDir); err != nil {
		return err
	}

	if err := store.Setup(); err != nil {
		return err
	}

	cfg, err := config.Read()
	if err != nil {
		return err
	}

	s, err := store.New(cfg)
	if err != nil {
		return err
	}
	s.UpdateNotes()

	return nil
}
