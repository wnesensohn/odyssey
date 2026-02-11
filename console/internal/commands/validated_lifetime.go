package commands

import (
	"errors"
	"time"
)

func ValidatedLifetime(issued, expires, now time.Time) (time.Duration, error) {
	if issued.IsZero() || issued.After(now) || !expires.After(now) || expires.Before(issued) {
		return 0, errors.New("invalid command time order")
	}
	duration := expires.Sub(issued)
	if duration > time.Minute {
		return 0, errors.New("command lifetime exceeds limit")
	}
	return duration, nil
}
