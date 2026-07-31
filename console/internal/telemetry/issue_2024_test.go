package telemetry

import (
	"testing"
	"time"
)

func TestIssue2024Nominal(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	cache, _ := NewCache(4)
	for i := 0; i < 4; i++ {
		_ = cache.Append(Sample{Channel: "pump", Unit: "kPa", Value: float64(i), Time: now.Add(time.Duration(i) * time.Second)})
	}
	if cache.RetainChannel("pump", 2) != nil || len(cache.History("pump")) != 2 {
		t.Fatal("retention failed")
	}
}

func TestIssue2024Boundary(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	cache, _ := NewCache(4)
	if cache.RetainChannel("pump", 0) == nil {
		t.Fatal("accepted zero retention")
	}
}

func TestIssue2024Invalid(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	cache, _ := NewCache(4)
	if cache.RetainChannel("missing", 2) != nil {
		t.Fatal("empty retention failed")
	}
}

func TestRetentionPersistsAcrossAppend(t *testing.T) {
	now := time.Unix(100, 0)
	cache, _ := NewCache(4)
	_ = cache.RetainChannel("pump", 2)
	for i := 0; i < 8; i++ {
		_ = cache.Append(Sample{Channel: "pump", Unit: "kPa", Value: float64(i), Time: now.Add(time.Duration(i) * time.Second)})
	}
	if len(cache.History("pump")) != 2 {
		t.Fatal("channel limit forgotten")
	}
	latest, _ := cache.Latest("pump")
	if latest.Value != 7 {
		t.Fatal("latest sample was removed")
	}
}

func TestChannelLimitDoesNotAffectOtherChannels(t *testing.T) {
	now := time.Unix(100, 0)
	c, _ := NewCache(4)
	_ = c.RetainChannel("pump", 1)
	for _, channel := range []string{"pump", "tank"} {
		for i := 0; i < 3; i++ {
			_ = c.Append(Sample{Channel: channel, Unit: "kPa", Time: now.Add(time.Duration(i) * time.Second)})
		}
	}
	if len(c.History("pump")) != 1 || len(c.History("tank")) != 3 {
		t.Fatal("retention leaked across channels")
	}
}
