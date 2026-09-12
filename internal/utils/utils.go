package utils

import (
	"fmt"
	"os"
)

func IsReadableDir(dir string) error {
	info, err := os.Stat(dir)
	if err != nil {
		return fmt.Errorf("could not stat dir %s: %w", dir, err)
	}

	if !info.IsDir() {
		return fmt.Errorf("not a directory %s", dir)
	}

	d, err := os.Open(dir)
	if err != nil {
		return fmt.Errorf("could not open dir %s: %w", dir, err)
	}
	defer d.Close()

	return nil
}
