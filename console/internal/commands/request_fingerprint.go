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
