package telemetry

import (
	"testing"
	"time"
)

func TestIssue2132Nominal(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	if SampleRate([]Sample{{Time: now}, {Time: now.Add(time.Second)}}) != 1 {
		t.Fatal("sample rate")
	}
}

func TestIssue2132Boundary(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	if SampleRate(nil) != 0 {
		t.Fatal("empty rate")
	}
}

func TestIssue2132Invalid(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	if SampleRate([]Sample{{Time: now}, {Time: now}}) != 0 {
		t.Fatal("duplicate clock rate")
	}
}
