package commands

import (
	"strings"
	"testing"
	"time"
)

func TestOperatorByteLimit(t *testing.T) {
	now := time.Unix(100, 0)
	c := Command{Kind: Safe, Operator: strings.Repeat("a", 129), Issued: now, Expires: now.Add(time.Second)}
	if Validate(c, now) == nil {
		t.Fatal("oversized identity accepted")
	}
	c.Operator = strings.Repeat("a", 128)
	if err := Validate(c, now); err != nil {
		t.Fatal(err)
	}
}

func TestOperatorLimitCountsUTF8Bytes(t *testing.T) {
	now := time.Unix(100, 0)
	c := Command{Kind: Safe, Operator: strings.Repeat("é", 65), Issued: now, Expires: now.Add(time.Second)}
	if Validate(c, now) == nil {
		t.Fatal("byte limit treated as rune count")
	}
}
