package commands

import (
	"testing"
	"time"
)

func TestIssue2123Nominal(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	value, err := RequestFingerprint(Command{Kind: Safe, Operator: "operator"})
	if err != nil || value != "safe|operator|0" {
		t.Fatal(value, err)
	}
}

func TestIssue2123Boundary(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	if _, err := RequestFingerprint(Command{}); err == nil {
		t.Fatal("missing identity accepted")
	}
}
