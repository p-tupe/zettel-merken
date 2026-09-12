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

	store, err := store.Load()
	if err != nil {
		return err
	}

	fmt.Println(cfg)

	v, err := store.Version()
	if err != nil {
		return err
	}
	fmt.Println("Sql Version:", v)

	return nil
}
