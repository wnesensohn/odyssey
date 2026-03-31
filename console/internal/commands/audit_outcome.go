package commands

import (
	"time"
)

type AuditOutcome struct {
	Operator string
	Kind     Kind
	Accepted bool
	Reason   string
	Time     time.Time
}

func Audit(command Command, now time.Time) AuditOutcome {
	result := AuditOutcome{Operator: command.Operator, Kind: command.Kind, Time: now}
	if err := Validate(command, now); err != nil {
		result.Reason = err.Error()
	} else {
		result.Accepted = true
		result.Reason = "accepted"
	}
	return result
}
