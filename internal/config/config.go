package config

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/p-tupe/zettel-merken/internal/utils"
)

type Config struct {
	// Parent notes directory list
	Notes []string `json:"notes"`
	// Files (or globs) to exlude
	Exclude []string `json:"exclude"`

	// Metadata, not inside actual json
	Path string `json:"-"`
}

func Create(notesDir string) (Config, error) {
	var cfg Config

	cfgPath, err := Path()
	if err != nil {
		return cfg, err
	}

	if utils.IsFileReadable(cfgPath) {
		return cfg, fmt.Errorf("config already exists, use `config` command to edit")
	}

	cfg = Config{
		Notes:   []string{notesDir},
		Exclude: []string{".*"},
		Path:    cfgPath,
	}

	cfgStr, err := json.MarshalIndent(cfg, "", "  ")
	if err != nil {
		return cfg, fmt.Errorf("unable to marshal default config: %w", err)
	}

	if err := os.WriteFile(cfg.Path, cfgStr, 0o644); err != nil {
		return cfg, fmt.Errorf("unable to write default config: %w", err)
	}

	return cfg, nil
}

func Path() (string, error) {
	userConfigDir, err := os.UserConfigDir()
	if err != nil {
		return "", fmt.Errorf("unable to open config dir at %s: %w", userConfigDir, err)
	}

	appConfigDir := filepath.Join(userConfigDir, "zettelmerken")
	if err := os.MkdirAll(appConfigDir, 0o755); err != nil {
		return "", fmt.Errorf("unable to create app config dir at %s: %w", appConfigDir, err)
	}

	return filepath.Join(userConfigDir, "zettelmerken", "config.json"), nil
}

func Read() (Config, error) {
	var cfg Config

	cfgPath, err := Path()
	if err != nil {
		return cfg, err
	}

	cfgFile, err := os.ReadFile(cfgPath)
	if err != nil {
		return cfg, fmt.Errorf("unable to open config file: %w", err)
	}

	err = json.Unmarshal(cfgFile, &cfg)
	if err != nil {
		return cfg, fmt.Errorf("unable to decode config json: %w", err)
	}

	cfg.Path = cfgPath

	return cfg, nil
}

func (cfg Config) Edit() error {
	editor := os.Getenv("VISUAL")
	if editor == "" {
		editor = os.Getenv("EDITOR")
		if editor == "" {
			return fmt.Errorf("no $VISUAL or $EDITOR set in environment")
		}
	}

	cmd := exec.Command(editor, cfg.Path)
	cmd.Stdin, cmd.Stdout, cmd.Stderr = os.Stdin, os.Stdout, os.Stderr
	return cmd.Run()
}
