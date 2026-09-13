package main

import (
	"github.com/p-tupe/zettel-merken/internal/config"
	"github.com/p-tupe/zettel-merken/internal/store"
)

func dailyRun() error {
	cfg, err := config.Read()
	if err != nil {
		return err
	}

	store, err := store.Create(cfg)
	if err != nil {
		return err
	}

	if err = store.UpdateNotes(); err != nil {
		return err
	}

	return nil
}
