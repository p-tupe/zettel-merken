package store

type Store struct{}

func Load() (*Store, error) {
	return &Store{}, nil
}
