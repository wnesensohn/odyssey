package events

import (
	"testing"
	"time"
)

func TestJournalRetentionAndSubscriberCancellation(t *testing.T) {
	journal, _ := NewJournal(2)
	stream, cancel := journal.Subscribe()
	for i := 0; i < 3; i++ {
		if _, err := journal.Append(Event{Time: time.Unix(100+int64(i), 0), Area: "power", Severity: "info", Message: "bus nominal"}); err != nil {
			t.Fatal(err)
		}
	}
	retained := journal.Since(0)
	if len(retained) != 2 || retained[0].ID != 2 {
		t.Fatal("journal retention changed")
	}
	if (<-stream).ID != 1 {
		t.Fatal("subscriber missed first event")
	}
	cancel()
	cancel()
	for range stream {
	}
	if _, err := journal.Append(Event{Time: time.Now(), Area: "power", Severity: "critical", Message: "undervoltage"}); err != nil {
		t.Fatal(err)
	}
}
func TestInvalidEventsDoNotConsumeIDs(t *testing.T) {
	journal, _ := NewJournal(10)
	if _, err := journal.Append(Event{}); err == nil {
		t.Fatal("accepted empty event")
	}
	event, err := journal.Append(Event{Time: time.Unix(100, 0), Area: "comms", Severity: "warning", Message: "link degraded"})
	if err != nil || event.ID != 1 {
		t.Fatal("invalid event consumed sequence")
	}
}
