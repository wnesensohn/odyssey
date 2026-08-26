package commands

import (
	"strings"
	"testing"
	"time"
)

func TestOperatorLabelBoundary(t *testing.T) {
	now := time.Unix(100, 0)
	c := Command{Kind: Safe, Operator: strings.Repeat("a", 256), Issued: now, Expires: now.Add(time.Second)}
	if err := Validate(c, now); err != nil {
		t.Fatal(err)
	}
	c.Operator += "a"
	if Validate(c, now) == nil {
		t.Fatal("oversized label accepted")
	}
}
