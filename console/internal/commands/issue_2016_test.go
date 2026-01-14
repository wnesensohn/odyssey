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
