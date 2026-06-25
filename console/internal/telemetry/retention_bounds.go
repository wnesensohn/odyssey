package telemetry

import (
	"time"
)

func (c *Cache) RetentionBounds(channel string) (time.Time, time.Time, int) {
	c.mu.RLock()
	defer c.mu.RUnlock()
	history := c.entries[channel]
	if len(history) == 0 {
		return time.Time{}, time.Time{}, 0
	}
	return history[0].Time, history[len(history)-1].Time, len(history)
}
