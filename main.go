package main

import "os"

func main() {
	cfgFile, err := os.Open("./config.example.json")
	if err != nil {
		panic(err)
	}

	cfg, err := NewConfig(cfgFile)
	if err != nil {
		panic(err)
	}

	db, err := NewConn()
	if err != nil {
		panic(err)
	}

	print(cfg, db)
}
