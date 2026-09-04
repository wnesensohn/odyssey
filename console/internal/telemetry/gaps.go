package telemetry

import (
	"errors"
	"time"
)

type Gap struct {
	Previous time.Time
	Next     time.Time
	Duration time.Duration
}

func (c *Cache) Gaps(channel string, maximumInterval time.Duration) ([]Gap, error) {
	if maximumInterval <= 0 {
		return nil, errors.New("positive gap interval required")
	}
	c.mu.RLock()
	defer c.mu.RUnlock()
	history := c.entries[channel]
	gaps := []Gap{}
	for i := 1; i < len(history); i++ {
		elapsed := history[i].Time.Sub(history[i-1].Time)
		if elapsed > maximumInterval {
			gaps = append(gaps, Gap{Previous: history[i-1].Time, Next: history[i].Time, Duration: elapsed})
		}
	}
	return gaps, nil
}
