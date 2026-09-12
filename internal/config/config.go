package config

import (
	"encoding/json"
	"fmt"
	"os"
)

// type Slack struct {
// 	Channel string `json:"channel"`
// }
//
// type Mailer struct {
// 	Host string `json:"host"`
// 	Port string `json:"port"`
// 	User string `json:"user"`
// 	Pass string `json:"pass"`
// }

type Cfg struct {
	Notes string `json:"notes"`
	// Mailer *Mailer `json:"mailer"`
	// Slack  *Slack  `json:"slack"`
}

func Read() (Cfg, error) {
	var cfg Cfg

	if len(os.Args) < 2 || os.Args[1] == "" {
		return cfg, fmt.Errorf("no config file path found")
	}

	cfgFile, err := os.Open(os.Args[1])
	if err != nil {
		return cfg, fmt.Errorf("unable to open config file: %w", err)
	}

	err = json.NewDecoder(cfgFile).Decode(&cfg)
	if err != nil {
		return cfg, fmt.Errorf("unable to decode config json: %w", err)
	}

	return cfg, nil
}
