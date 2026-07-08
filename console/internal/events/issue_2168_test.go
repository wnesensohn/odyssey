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

func TestIssue2168Boundary(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	if CommandOutcome(1, "operator", false, now).Severity != "warning" {
		t.Fatal("rejected severity")
	}
}

func TestIssue2168Invalid(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	if CommandOutcome(1, "operator", false, now).Area != "commands" {
		t.Fatal("wrong event area")
	}
}

func TestOutcomeJournalAssignsItsOwnSequence(t *testing.T) {
	j, _ := NewJournal(3)
	event, err := j.Append(CommandOutcome(99, "operator", true, time.Unix(100, 0)))
	if err != nil || event.ID != 1 {
		t.Fatal(event, err)
	}
}
