package main

type DB struct{}

func NewConn() (*DB, error) {
	db := &DB{}

	return db, nil
}
