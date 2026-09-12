package initialize

import (
	"fmt"
	"os"

	"github.com/p-tupe/zettel-merken/internal/config"
)

func Setup() error {
	if len(os.Args) < 3 {
		return fmt.Errorf("need a notes directory to initialize")
	}

	err := config.Setup(os.Args[2])
	if err != nil {
		return err
	}

	// Setup store
	// Scan notes into store
	return nil
}
