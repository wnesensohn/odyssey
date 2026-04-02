package commands

import (
	"testing"
	"time"
)

func TestIssue2087Nominal(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	result := Audit(Command{Kind: Safe, Operator: "operator", Issued: now, Expires: now.Add(time.Second)}, now)
	if !result.Accepted {
		t.Fatal(result)
	}
}

func TestIssue2087Boundary(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	result := Audit(Command{}, now)
	if result.Accepted || result.Reason == "" {
		t.Fatal("rejection not recorded")
	}
}

func TestIssue2087Invalid(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	result := Audit(Command{Kind: Safe, Operator: "operator", Value: 1, Issued: now, Expires: now.Add(time.Second)}, now)
	if result.Accepted {
		t.Fatal("invalid safe value accepted")
	}
}
