package main

import (
	"bytes"
	"encoding/json"
	"testing"
)

func TestConfig(t *testing.T) {
	expectCfg := Config{
		NOTE_DIRS: []string{"/some/path"},
		EMAIL: struct {
			USER string
			PASS string
			HOST string
			PORT int
		}{
			USER: "user",
			PASS: "pass",
			HOST: "host",
			PORT: 100,
		},
		RECEIVERS: []string{"some@email.com"},
	}

	jsonByte, err := json.Marshal(expectCfg)
	if err != nil {
		t.Error(err)
	}

	gotCfg, err := NewConfig(bytes.NewReader(jsonByte))
	if err != nil {
		t.Error(err)
	}

	if gotCfg.MAX_NOTES_PER_MAIL == 0 || len(gotCfg.INCLUDE_EXT) == 0 || len(gotCfg.SCHEDULE_DAYS) == 0 {
		t.Error("Expected default values not present")
	}
}
