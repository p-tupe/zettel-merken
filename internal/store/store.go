package store

import (
	"database/sql"
	"fmt"
	"os"
	"path/filepath"

	"github.com/p-tupe/zettel-merken/internal/config"
	"github.com/p-tupe/zettel-merken/internal/utils"
	_ "modernc.org/sqlite"
)

type Store struct {
	db  *sql.DB
	cfg config.Config
}

func New(cfg config.Config) (*Store, error) {
	db, err := getDB()
	if err != nil {
		return nil, err
	}

	return &Store{db, cfg}, nil
}

func Setup() error {
	db, err := getDB()
	if err != nil {
		return err
	}

	createNotes := `create table if not exists notes ( 
		id integer primary key,
		title text not null unique,
		hash text not null unique,
		size integer not null,
		mtime text not null,
		deleted_at text
	);`

	if _, err := db.Exec(createNotes); err != nil {
		return fmt.Errorf("could not create table: %w", err)
	}

	return nil
}

func getDB() (*sql.DB, error) {
	cfgPath, err := config.Path()
	if err != nil {
		return nil, err
	}

	dbPath := filepath.Join(filepath.Dir(cfgPath), "zettelmerken.db")
	db, err := sql.Open("sqlite", dbPath)
	if err != nil {
		return nil, fmt.Errorf("could not open database at %s: %w", dbPath, err)
	}

	return db, nil
}

func (s *Store) UpdateNotes() error {
	entries := make([]os.DirEntry, 0, 10)
	for _, notesDir := range s.cfg.Notes {
		newEntries, err := utils.WalkNotesDir(notesDir, s.cfg.Exclude)
		if err != nil {
			return err
		}

		entries = append(entries, newEntries...)
	}

	for _, e := range entries {
		fmt.Println(e.Name())
	}

	return nil
}

func (s *Store) Version() (string, error) {
	row := s.db.QueryRow(`SELECT sqlite_version();`)
	var v string
	if err := row.Scan(&v); err != nil {
		return "", fmt.Errorf("could not scan row: %w", err)
	}
	return v, nil
}
