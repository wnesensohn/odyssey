package health

import (
	"testing"
	"time"
)

func TestHealthRequiresFreshMonotonicObservations(t *testing.T) {
	registry := NewRegistry()
	now := time.Unix(100, 0)
	if _, healthy := registry.Snapshot(now, time.Second); healthy {
		t.Fatal("empty registry reported healthy")
	}
	if err := registry.Update(Probe{Name: "ground-link", Healthy: true, Checked: now}); err != nil {
		t.Fatal(err)
	}
	if _, healthy := registry.Snapshot(now, time.Second); !healthy {
		t.Fatal("fresh probe unhealthy")
	}
	if _, healthy := registry.Snapshot(now.Add(2*time.Second), time.Second); healthy {
		t.Fatal("stale probe remained healthy")
	}
	if registry.Update(Probe{Name: "ground-link", Healthy: true, Checked: now.Add(-time.Second)}) == nil {
		t.Fatal("accepted reordered health probe")
	}
}
