package initialize

import (
	"github.com/p-tupe/zettel-merken/internal/config"
	"github.com/p-tupe/zettel-merken/internal/store"
	"github.com/p-tupe/zettel-merken/internal/utils"
)

func Setup(notesDir string) error {
	if err := utils.IsReadableDir(notesDir); err != nil {
		return err
	}

	if err := config.Setup(notesDir); err != nil {
		return err
	}

	if err := store.Setup(); err != nil {
		return err
	}

	// TODO: Scan notes into store

	return nil
}
