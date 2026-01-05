package commands

import (
	"errors"
	"math"
	"sync"
	"time"
)

type Kind string

const (
	Safe             Kind = "safe"
	ArmEngine        Kind = "arm-engine"
	SetThrottle      Kind = "set-throttle"
	StopEngine       Kind = "stop-engine"
	SetHeater        Kind = "set-heater"
	AcknowledgeFault Kind = "acknowledge-fault"
)

type Command struct {
	ID       uint64    `json:"id"`
	Kind     Kind      `json:"kind"`
	Value    float64   `json:"value"`
	Operator string    `json:"operator"`
	Issued   time.Time `json:"issued"`
	Expires  time.Time `json:"expires"`
}

func Validate(command Command, now time.Time) error {
	if command.Operator == "" || command.Issued.IsZero() || command.Issued.After(now) || command.Expires.Before(now) {
		return errors.New("command identity or timing is invalid")
	}
	if command.Expires.Sub(command.Issued) > time.Minute || command.Expires.Before(command.Issued) {
		return errors.New("command lifetime exceeds limit")
	}
	if math.IsNaN(command.Value) || math.IsInf(command.Value, 0) {
		return errors.New("command value must be finite")
	}
	switch command.Kind {
	case SetThrottle:
		if command.Value < 0 || command.Value > 1 {
			return errors.New("throttle outside range")
		}
	case SetHeater:
		if command.Value < 0 || command.Value > 5000 {
			return errors.New("heater power outside range")
		}
	case AcknowledgeFault:
		if command.Value < 0 || command.Value > 65535 || math.Trunc(command.Value) != command.Value {
			return errors.New("invalid fault identifier")
		}
	case Safe, ArmEngine, StopEngine:
		if command.Value != 0 {
			return errors.New("command does not accept a value")
		}
	default:
		return errors.New("unknown command")
	}
	return nil
}

type Service struct {
	mu       sync.Mutex
	nextID   uint64
	queue    []Command
	capacity int
}

func NewService(capacity int) (*Service, error) {
	if capacity < 1 || capacity > 256 {
		return nil, errors.New("invalid command capacity")
	}
	return &Service{capacity: capacity, nextID: 1}, nil
}

func (s *Service) Submit(command Command, now time.Time) (Command, error) {
	if err := Validate(command, now); err != nil {
		return Command{}, err
	}
	s.mu.Lock()
	defer s.mu.Unlock()
	if len(s.queue) >= s.capacity {
		return Command{}, errors.New("command queue is full")
	}
	command.ID = s.nextID
	s.nextID++
	s.queue = append(s.queue, command)
	return command, nil
}

func (s *Service) Next(now time.Time) (Command, bool) {
	s.mu.Lock()
	defer s.mu.Unlock()
	for len(s.queue) > 0 {
		command := s.queue[0]
		s.queue = s.queue[1:]
		if !command.Expires.Before(now) {
			return command, true
		}
	}
	return Command{}, false
}

func (s *Service) Pending() int {
	s.mu.Lock()
	defer s.mu.Unlock()
	return len(s.queue)
}
