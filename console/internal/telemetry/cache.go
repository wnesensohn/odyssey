package telemetry

import (
	"errors"
	"math"
	"sort"
	"sync"
	"time"
)

type Sample struct {
	Channel string    `json:"channel"`
	Unit    string    `json:"unit"`
	Value   float64   `json:"value"`
	Time    time.Time `json:"time"`
	Quality string    `json:"quality"`
}

type Cache struct {
	mu       sync.RWMutex
	entries  map[string][]Sample
	capacity int
}

func NewCache(capacity int) (*Cache, error) {
	if capacity < 1 || capacity > 4096 {
		return nil, errors.New("invalid telemetry capacity")
	}
	return &Cache{entries: make(map[string][]Sample), capacity: capacity}, nil
}

func (c *Cache) Append(sample Sample) error {
	if sample.Channel == "" || sample.Unit == "" || math.IsNaN(sample.Value) || math.IsInf(sample.Value, 0) || sample.Time.IsZero() {
		return errors.New("invalid telemetry sample")
	}
	c.mu.Lock()
	defer c.mu.Unlock()
	history := c.entries[sample.Channel]
	if len(history) > 0 && sample.Time.Before(history[len(history)-1].Time) {
		return errors.New("telemetry time moved backwards")
	}
	if len(history) == c.capacity {
		history = append([]Sample(nil), history[1:]...)
	}
	c.entries[sample.Channel] = append(history, sample)
	return nil
}

func (c *Cache) Snapshot(now time.Time, maximumAge time.Duration) []Sample {
	c.mu.RLock()
	defer c.mu.RUnlock()
	result := make([]Sample, 0, len(c.entries))
	for _, history := range c.entries {
		sample := history[len(history)-1]
		if !sample.Time.After(now) && now.Sub(sample.Time) <= maximumAge {
			result = append(result, sample)
		}
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Channel < result[j].Channel })
	return result
}

func (c *Cache) History(channel string) []Sample {
	c.mu.RLock()
	defer c.mu.RUnlock()
	return append([]Sample(nil), c.entries[channel]...)
}

func (c *Cache) Latest(channel string) (Sample, bool) {
	c.mu.RLock()
	defer c.mu.RUnlock()
	history := c.entries[channel]
	if len(history) == 0 {
		return Sample{}, false
	}
	return history[len(history)-1], true
}
