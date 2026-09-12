package store

import (
	"database/sql"
	"fmt"
	"path/filepath"

	"github.com/p-tupe/zettel-merken/internal/config"
	_ "modernc.org/sqlite"
)

type Store struct {
	db *sql.DB
}

func Setup() error {
	s, err := Load()
	if err != nil {
		return err
	}

	createNotes := `
	create table notes
	`

	if _, err := s.db.Exec(createNotes); err != nil {
		return fmt.Errorf("could not create table: %w", err)
	}

	return nil
}

func Load() (*Store, error) {
	cfgPath, err := config.Path()
	if err != nil {
		return nil, err
	}

	dbPath := filepath.Join(filepath.Dir(cfgPath), "zettelmerken.db")
	db, err := sql.Open("sqlite", dbPath)
	if err != nil {
		return nil, fmt.Errorf("could not open database at %s: %w", dbPath, err)
	}

	return &Store{db}, nil
}

func (s *Store) Version() (string, error) {
	row := s.db.QueryRow(`SELECT sqlite_version();`)
	var v string
	if err := row.Scan(&v); err != nil {
		return "", fmt.Errorf("could not scan row: %w", err)
	}
	return v, nil
}
