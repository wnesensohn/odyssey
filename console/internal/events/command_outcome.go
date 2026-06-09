package events

import (
	"time"
)

func CommandOutcome(id uint64, operator string, accepted bool, now time.Time) Event {
	severity := "info"
	message := "Command completed"
	if !accepted {
		severity = "warning"
		message = "Command rejected"
	}
	return Event{ID: id, Time: now, Area: "commands", Severity: severity, Message: message + " for " + operator}
}
