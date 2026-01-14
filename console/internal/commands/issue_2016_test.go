package commands

import (
	"testing"
	"time"
)

func TestIssue2016Nominal(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	value, err := ValidatedLifetime(now, now.Add(time.Second), now)
	if err != nil || value != time.Second {
		t.Fatal(value, err)
	}
}

func TestIssue2016Boundary(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	if _, err := ValidatedLifetime(now, now.Add(2*time.Minute), now); err == nil {
		t.Fatal("accepted excess lifetime")
	}
}

func TestIssue2016Invalid(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	if _, err := ValidatedLifetime(now.Add(time.Second), now, now); err == nil {
		t.Fatal("accepted reversed time")
	}
}
