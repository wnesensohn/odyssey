package health

import (
	"testing"
	"time"
)

func TestIssue2104Nominal(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	if ClassifyProbe(Probe{Checked: now, Healthy: true}, now, time.Second) != "healthy" {
		t.Fatal("fresh health")
	}
}

func TestIssue2104Boundary(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	if ClassifyProbe(Probe{Checked: now, Healthy: false}, now, time.Second) != "degraded" {
		t.Fatal("degradation lost")
	}
}

func TestIssue2104Invalid(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	if ClassifyProbe(Probe{Checked: now.Add(time.Second), Healthy: true}, now, time.Second) != "stale" {
		t.Fatal("future health accepted")
	}
}
