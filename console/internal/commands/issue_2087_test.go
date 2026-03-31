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
