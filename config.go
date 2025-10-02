package main

import (
	"encoding/json"
	"errors"
	"io"
)

type Config struct {
	// The directories where notes reside
	NOTE_DIRS []string

	// The host (sender) email credentials
	EMAIL struct {
		USER string
		PASS string
		HOST string
		PORT int
	}

	// A list of recipient emails
	RECEIVERS []string

	// Files inside NOTE_DIRS to be ignored
	IGNORE_FILES []string `json:",omitempty"`

	// Sub-dirs of NOTE_DIRS to be ignored
	IGNORE_DIRS []string `json:",omitempty"`

	// Maximum notes to send in a single email (default 10)
	MAX_NOTES_PER_MAIL int `json:",omitempty"`

	// Only notes with listed extensions to be included
	// default {".md", ".txt", ".docx"}
	INCLUDE_EXT []string `json:",omitempty"`

	// Set a custom schedule for spaced repeatitions
	// by default sends a note after 1, 3, 7, 14, 30, 60, 90, 180, 360 days
	SCHEDULE_DAYS []int `json:",omitempty"`
}

func NewConfig(reader io.Reader) (*Config, error) {
	cfg := &Config{
		MAX_NOTES_PER_MAIL: 10,
		INCLUDE_EXT:        []string{".md", ".txt", ".docx", ".csv"},
		SCHEDULE_DAYS:      []int{1, 3, 7, 14, 30, 60, 90, 180, 360},
	}

	data, err := io.ReadAll(reader)
	if err != nil {
		return nil, err
	}

	err = json.Unmarshal(data, cfg)
	if err != nil {
		return nil, err
	}

	if len(cfg.NOTE_DIRS) == 0 {
		return nil, errors.New("Missing required NOTE_DIRS value")
	}

	if cfg.EMAIL.HOST == "" || cfg.EMAIL.PASS == "" ||
		cfg.EMAIL.PORT == 0 || cfg.EMAIL.USER == "" {
		return nil, errors.New("Missing required EMAIL values")
	}

	if len(cfg.RECEIVERS) == 0 {
		return nil, errors.New("Missing required RECEIVERS value")
	}

	return cfg, nil
}
