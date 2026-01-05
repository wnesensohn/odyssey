package commands

import (
	"math"
	"testing"
	"time"
)

func nominal(now time.Time) Command {
	return Command{Kind: SetThrottle, Value: 0.5, Operator: "flight-operator", Issued: now, Expires: now.Add(5 * time.Second)}
}
func TestCommandKindsHaveDistinctValueRanges(t *testing.T) {
	now := time.Unix(100, 0)
	cases := []struct {
		kind  Kind
		value float64
		valid bool
	}{
		{Safe, 0, true}, {Safe, 1, false}, {SetThrottle, 0, true}, {SetThrottle, 1, true}, {SetThrottle, 1.1, false},
		{SetHeater, 5000, true}, {SetHeater, 5001, false}, {AcknowledgeFault, 12, true}, {AcknowledgeFault, 12.5, false}, {"unknown", 0, false},
	}
	for _, test := range cases {
		command := nominal(now)
		command.Kind = test.kind
		command.Value = test.value
		if (Validate(command, now) == nil) != test.valid {
			t.Fatalf("unexpected validation for %s=%v", test.kind, test.value)
		}
	}
}
func TestMalformedAndExpiredCommandsAreRejected(t *testing.T) {
	now := time.Unix(100, 0)
	for _, value := range []float64{math.NaN(), math.Inf(1), math.Inf(-1)} {
		command := nominal(now)
		command.Value = value
		if Validate(command, now) == nil {
			t.Fatal("accepted nonfinite value")
		}
	}
	command := nominal(now)
	command.Operator = ""
	if Validate(command, now) == nil {
		t.Fatal("accepted anonymous operator")
	}
	command = nominal(now)
	command.Issued = now.Add(time.Second)
	if Validate(command, now) == nil {
		t.Fatal("accepted future command")
	}
	command = nominal(now)
	command.Expires = now.Add(-time.Second)
	if Validate(command, now) == nil {
		t.Fatal("accepted expired command")
	}
}
func TestQueueCapacityAndExpiredDrain(t *testing.T) {
	now := time.Unix(100, 0)
	service, _ := NewService(2)
	first, err := service.Submit(nominal(now), now)
	if err != nil {
		t.Fatal(err)
	}
	second, _ := service.Submit(nominal(now), now)
	if second.ID != first.ID+1 {
		t.Fatal("command IDs are not sequential")
	}
	if _, err := service.Submit(nominal(now), now); err == nil {
		t.Fatal("accepted command beyond capacity")
	}
	if _, ok := service.Next(now.Add(10 * time.Second)); ok {
		t.Fatal("released expired command")
	}
	if service.Pending() != 0 {
		t.Fatal("expired queue did not drain")
	}
}
