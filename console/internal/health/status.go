package health

import (
	"errors"
	"sort"
	"sync"
	"time"
)

type Probe struct {
	Name    string    `json:"name"`
	Healthy bool      `json:"healthy"`
	Detail  string    `json:"detail"`
	Checked time.Time `json:"checked"`
}
type Registry struct {
	mu     sync.RWMutex
	probes map[string]Probe
}

func NewRegistry() *Registry { return &Registry{probes: make(map[string]Probe)} }
func (r *Registry) Update(probe Probe) error {
	if probe.Name == "" || probe.Checked.IsZero() {
		return errors.New("invalid health probe")
	}
	r.mu.Lock()
	defer r.mu.Unlock()
	if old, ok := r.probes[probe.Name]; ok && probe.Checked.Before(old.Checked) {
		return errors.New("health observation moved backwards")
	}
	r.probes[probe.Name] = probe
	return nil
}
func (r *Registry) Snapshot(now time.Time, maximumAge time.Duration) ([]Probe, bool) {
	r.mu.RLock()
	defer r.mu.RUnlock()
	probes := []Probe{}
	healthy := len(r.probes) > 0
	for _, probe := range r.probes {
		if probe.Checked.After(now) || now.Sub(probe.Checked) > maximumAge {
			probe.Healthy = false
			probe.Detail = "stale"
		}
		healthy = healthy && probe.Healthy
		probes = append(probes, probe)
	}
	sort.Slice(probes, func(i, j int) bool { return probes[i].Name < probes[j].Name })
	return probes, healthy
}
