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
