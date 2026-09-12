package config

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

type Config struct {
	// Parent notes directory list
	Notes []string `json:"notes"`
	// Files (or globs) to exlude
	Exclude []string `json:"exclude"`
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

	return filepath.Join(appConfigDir, "config.json"), nil
}

func Read() (Config, error) {
	var cfg Config

	cfgPath, err := Path()
	if err != nil {
		return cfg, err
	}

	cfgFile, err := os.Open(cfgPath)
	if err != nil {
		return cfg, fmt.Errorf("unable to open config file: %w", err)
	}
	defer cfgFile.Close()

	err = json.NewDecoder(cfgFile).Decode(&cfg)
	if err != nil {
		return cfg, fmt.Errorf("unable to decode config json: %w", err)
	}

	return cfg, nil
}

func Edit() error {
	cfgPath, err := Path()
	if err != nil {
		return err
	}

	editor := os.Getenv("VISUAL")
	if editor == "" {
		editor = os.Getenv("EDITOR")
		if editor == "" {
			return errors.New("no $VISUAL or $EDITOR set in environment")
		}
	}

	cmd := exec.Command(editor, cfgPath)
	cmd.Stdin, cmd.Stdout, cmd.Stderr = os.Stdin, os.Stdout, os.Stderr
	if err := cmd.Run(); err != nil {
		return err
	}

	return nil
}

func Setup(notesDir string) error {
	cfgPath, err := Path()
	if err != nil {
		return err
	}

	cfgFile, err := os.Create(cfgPath)
	if err != nil {
		return fmt.Errorf("could not create config file: %w", err)
	}
	defer cfgFile.Close()

	defaultCfg := Config{
		Notes:   []string{notesDir},
		Exclude: []string{".git"},
	}

	err = json.NewEncoder(cfgFile).Encode(defaultCfg)
	if err != nil {
		return fmt.Errorf("unable to write default config: %w", err)
	}

	return nil
}
