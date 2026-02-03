package telemetry

import (
	"errors"
)

func (c *Cache) RetainChannel(channel string, limit int) error {
	if limit < 1 || limit > c.capacity {
		return errors.New("invalid channel retention")
	}
	c.mu.Lock()
	defer c.mu.Unlock()
	history := c.entries[channel]
	if len(history) > limit {
		c.entries[channel] = append([]Sample(nil), history[len(history)-limit:]...)
	}
	return nil
}
