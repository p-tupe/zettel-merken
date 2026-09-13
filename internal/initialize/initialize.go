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

	cfg, err := config.Create(notesDir)
	if err != nil {
		return err
	}

	st, err := store.Create(cfg)
	if err != nil {
		return err
	}

	if err := st.UpdateNotes(); err != nil {
		return err
	}

	return nil
}
