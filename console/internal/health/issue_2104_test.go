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
