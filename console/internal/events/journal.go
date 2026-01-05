package events

import (
	"errors"
	"sync"
	"time"
)

type Event struct {
	ID       uint64    `json:"id"`
	Time     time.Time `json:"time"`
	Area     string    `json:"area"`
	Severity string    `json:"severity"`
	Message  string    `json:"message"`
}

type Journal struct {
	mu             sync.RWMutex
	entries        []Event
	nextID         uint64
	capacity       int
	subscribers    map[uint64]chan Event
	nextSubscriber uint64
}

func NewJournal(capacity int) (*Journal, error) {
	if capacity < 1 || capacity > 10000 {
		return nil, errors.New("invalid journal capacity")
	}
	return &Journal{capacity: capacity, nextID: 1, subscribers: make(map[uint64]chan Event)}, nil
}

func (j *Journal) Append(event Event) (Event, error) {
	if event.Time.IsZero() || event.Area == "" || event.Message == "" {
		return Event{}, errors.New("invalid event")
	}
	if event.Severity != "info" && event.Severity != "warning" && event.Severity != "critical" {
		return Event{}, errors.New("invalid severity")
	}
	j.mu.Lock()
	defer j.mu.Unlock()
	event.ID = j.nextID
	j.nextID++
	if len(j.entries) == j.capacity {
		j.entries = append([]Event(nil), j.entries[1:]...)
	}
	j.entries = append(j.entries, event)
	for _, subscriber := range j.subscribers {
		select {
		case subscriber <- event:
		default:
		}
	}
	return event, nil
}

func (j *Journal) Since(id uint64) []Event {
	j.mu.RLock()
	defer j.mu.RUnlock()
	result := []Event{}
	for _, event := range j.entries {
		if event.ID > id {
			result = append(result, event)
		}
	}
	return result
}

func (j *Journal) Subscribe() (<-chan Event, func()) {
	j.mu.Lock()
	id := j.nextSubscriber
	j.nextSubscriber++
	channel := make(chan Event, 32)
	j.subscribers[id] = channel
	j.mu.Unlock()
	var once sync.Once
	return channel, func() { once.Do(func() { j.mu.Lock(); delete(j.subscribers, id); close(channel); j.mu.Unlock() }) }
}
