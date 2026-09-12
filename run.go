package main

import (
	"fmt"

	"github.com/p-tupe/zettel-merken/internal/config"
	"github.com/p-tupe/zettel-merken/internal/store"
)

func dailyRun() error {
	cfg, err := config.Read()
	if err != nil {
		return err
	}

	store, err := store.New(cfg)
	if err != nil {
		return err
	}

	if err = store.UpdateNotes(); err != nil {
		return err
	}

	v, err := store.Version()
	if err != nil {
		return err
	}

	fmt.Println(cfg)
	fmt.Println("Sql Version:", v)

	return nil
}
