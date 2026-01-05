package session

import (
	"crypto/rand"
	"encoding/hex"
	"errors"
	"sync"
	"time"
)

type Role string

const (
	Observer   Role = "observer"
	Controller Role = "controller"
)

type Session struct {
	Token    string
	Operator string
	Role     Role
	Expires  time.Time
}
type Store struct {
	mu       sync.Mutex
	entries  map[string]Session
	lifetime time.Duration
}

func NewStore(lifetime time.Duration) (*Store, error) {
	if lifetime <= 0 || lifetime > 24*time.Hour {
		return nil, errors.New("invalid session lifetime")
	}
	return &Store{entries: make(map[string]Session), lifetime: lifetime}, nil
}

func (s *Store) Create(operator string, role Role, now time.Time) (Session, error) {
	if operator == "" || (role != Observer && role != Controller) {
		return Session{}, errors.New("invalid operator identity")
	}
	random := make([]byte, 32)
	if _, err := rand.Read(random); err != nil {
		return Session{}, err
	}
	value := Session{Token: hex.EncodeToString(random), Operator: operator, Role: role, Expires: now.Add(s.lifetime)}
	s.mu.Lock()
	s.entries[value.Token] = value
	s.mu.Unlock()
	return value, nil
}

func (s *Store) Lookup(token string, now time.Time) (Session, bool) {
	s.mu.Lock()
	defer s.mu.Unlock()
	value, ok := s.entries[token]
	if !ok || !now.Before(value.Expires) {
		delete(s.entries, token)
		return Session{}, false
	}
	return value, true
}

func (s *Store) Revoke(token string) { s.mu.Lock(); delete(s.entries, token); s.mu.Unlock() }
