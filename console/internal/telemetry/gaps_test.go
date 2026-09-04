package telemetry

import (
	"testing"
	"time"
)

func TestGapMarkersRetainBothEndpoints(t *testing.T) {
	now := time.Unix(100, 0)
	c, _ := NewCache(3)
	for _, offset := range []time.Duration{0, time.Second, 5 * time.Second} {
		_ = c.Append(Sample{Channel: "pump", Unit: "kPa", Time: now.Add(offset)})
	}
	gaps, err := c.Gaps("pump", 2*time.Second)
	if err != nil || len(gaps) != 1 || gaps[0].Previous != now.Add(time.Second) || gaps[0].Duration != 4*time.Second {
		t.Fatal(gaps, err)
	}
}
