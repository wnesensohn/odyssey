package events

import (
	"testing"
	"time"
)

func TestIssue2168Nominal(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	if CommandOutcome(1, "operator", true, now).Severity != "info" {
		t.Fatal("accepted outcome")
	}
}
