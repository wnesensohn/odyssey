package events

import (
	"testing"
	"time"
)

func TestIssue2063Nominal(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	journal, _ := NewJournal(1)
	for i := 0; i < 2; i++ {
		_, _ = journal.Append(Event{Time: now, Area: "power", Severity: "info", Message: "nominal"})
	}
	events, gap := journal.SinceWithGap(0)
	if !gap || len(events) != 1 {
		t.Fatal("gap missing")
	}
}
