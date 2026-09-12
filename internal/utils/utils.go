package utils

import (
	"fmt"
	"io/fs"
	"log/slog"
	"os"
	"path/filepath"
)

func IsReadableDir(dir string) bool {
	info, err := os.Stat(dir)
	if err != nil {
		return false
	}

	if !info.IsDir() {
		return false
	}

	d, err := os.Open(dir)
	if err != nil {
		return false
	}
	defer d.Close()

	return true
}

func IsFileReadable(file string) bool {
	info, err := os.Stat(file)
	if err != nil {
		return false
	}

	if info.IsDir() {
		return false
	}

	d, err := os.Open(file)
	if err != nil {
		return false
	}
	defer d.Close()

	return true
}

func WalkNotesDir(dir string, exclude []string) ([]os.DirEntry, error) {
	entries := make([]os.DirEntry, 0, 10)

	if err := filepath.WalkDir(dir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			slog.Warn("skipping path due to error", "path", path, "err", err)
			if d.IsDir() {
				return fs.SkipDir
			}

			return nil
		}

		if path == dir {
			return nil
		}

		for _, p := range exclude {
			if matched, err := filepath.Match(p, d.Name()); err != nil {
				slog.Warn("skipping dir due to error", "dir", d.Name(), "err", err)
				if d.IsDir() {
					return fs.SkipDir
				}
			} else {
				if matched {
					if d.IsDir() {
						return fs.SkipDir
					}
					return nil
				}
			}
		}

		if d.IsDir() {
			return nil
		}

		entries = append(entries, d)
		return nil
	}); err != nil {
		return nil, fmt.Errorf("could not walk dir %s: %w", dir, err)
	}

	return entries, nil
}
