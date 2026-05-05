package commands

import (
	"errors"
	"math"
	"strconv"
	"sync"
	"time"
)

func RequestFingerprint(command Command) (string, error) {
	if command.Operator == "" || math.IsNaN(command.Value) || math.IsInf(command.Value, 0) {
		return "", errors.New("invalid idempotency input")
	}
	return string(command.Kind) + "|" + command.Operator + "|" + strconv.FormatFloat(command.Value, 'g', -1, 64), nil
}

// RequestMemo bounds idempotency records by lifetime; a key cannot change payload.
type RequestMemo struct {
	mu       sync.Mutex
	entries  map[string]memoEntry
	capacity int
}
type memoEntry struct {
	fingerprint string
	result      Command
	expires     time.Time
}

func NewRequestMemo(capacity int) (*RequestMemo, error) {
	if capacity < 1 || capacity > 4096 {
		return nil, errors.New("invalid memo capacity")
	}
	return &RequestMemo{entries: make(map[string]memoEntry), capacity: capacity}, nil
}
func (m *RequestMemo) Submit(key string, command Command, service *Service, now time.Time) (Command, error) {
	if key == "" {
		return Command{}, errors.New("idempotency key required")
	}
	fingerprint, err := RequestFingerprint(command)
	if err != nil {
		return Command{}, err
	}
	m.mu.Lock()
	defer m.mu.Unlock()
	for key, entry := range m.entries {
		if !entry.expires.After(now) {
			delete(m.entries, key)
		}
	}
	if entry, ok := m.entries[key]; ok {
		if entry.fingerprint != fingerprint {
			return Command{}, errors.New("key reused with another command")
		}
		return entry.result, nil
	}
	if len(m.entries) >= m.capacity {
		return Command{}, errors.New("memo capacity exhausted")
	}
	result, err := service.Submit(command, now)
	if err != nil {
		return Command{}, err
	}
	m.entries[key] = memoEntry{fingerprint: fingerprint, result: result, expires: command.Expires}
	return result, nil
}
