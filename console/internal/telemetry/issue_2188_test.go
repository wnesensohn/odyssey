package telemetry

import (
	"testing"
	"time"
)

func TestIssue2188Nominal(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	cache, _ := NewCache(2)
	_ = cache.Append(Sample{Channel: "pump", Unit: "kPa", Time: now})
	a, b, count := cache.RetentionBounds("pump")
	if count != 1 || a != now || b != now {
		t.Fatal("retention bounds")
	}
}

func TestIssue2188Boundary(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	cache, _ := NewCache(2)
	a, b, count := cache.RetentionBounds("missing")
	if count != 0 || !a.IsZero() || !b.IsZero() {
		t.Fatal("empty bounds")
	}
}

func TestIssue2188Invalid(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	cache, _ := NewCache(1)
	for i := 0; i < 2; i++ {
		_ = cache.Append(Sample{Channel: "pump", Unit: "kPa", Time: now.Add(time.Duration(i) * time.Second)})
	}
	_, _, count := cache.RetentionBounds("pump")
	if count != 1 {
		t.Fatal("capacity ignored")
	}
}
