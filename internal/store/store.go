package store

import (
	"database/sql"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

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

func Create(cfg config.Config) (*Store, error) {
	s, err := New(cfg)
	if err != nil {
		return nil, err
	}

	createNotes := `create table if not exists notes ( 
		id integer primary key,
		title text not null unique,
		last_seen text not null
	);`

	if _, err := s.db.Exec(createNotes); err != nil {
		return s, fmt.Errorf("could not create table: %w", err)
	}

	return s, nil
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

	var upsertSQL strings.Builder
	upsertSQL.WriteString("insert into notes (title, last_seen) values")
	upsertSQL.WriteString(strings.TrimSuffix(strings.Repeat("(?,?),", len(entries)), ","))
	upsertSQL.WriteString("on conflict(title) do update set last_seen = excluded.last_seen;")

	stmt, err := s.db.Prepare(upsertSQL.String())
	if err != nil {
		return err
	}

	args := make([]any, 0, len(entries)*2)
	for _, e := range entries {
		args = append(args, e.Name(), time.Now().UTC().Format(time.RFC3339))
	}

	if _, err := stmt.Exec(args...); err != nil {
		return fmt.Errorf("could not execute statement %w", err)
	}

	s.countNotes()
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

// debug only
func (s *Store) countNotes() {
	r := s.db.QueryRow(`select count(*) from notes;`)
	var count string
	if err := r.Scan(&count); err != nil {
		panic(err)
	} else {
		fmt.Println("Number of notes:", count)
	}
}
