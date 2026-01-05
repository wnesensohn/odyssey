package telemetry

import (
	"math"
	"testing"
	"time"
)

func TestHistoryIsBoundedAndReturnedAsACopy(t *testing.T) {
	cache, _ := NewCache(2)
	start := time.Unix(100, 0)
	for i := 0; i < 3; i++ {
		if err := cache.Append(Sample{Channel: "pump.pressure", Unit: "kPa", Value: float64(i), Time: start.Add(time.Duration(i) * time.Second), Quality: "good"}); err != nil {
			t.Fatal(err)
		}
	}
	history := cache.History("pump.pressure")
	if len(history) != 2 || history[0].Value != 1 {
		t.Fatal("wrong history retention")
	}
	history[0].Value = 999
	if cache.History("pump.pressure")[0].Value == 999 {
		t.Fatal("history escaped cache lock")
	}
	latest, ok := cache.Latest("pump.pressure")
	if !ok || latest.Value != 2 {
		t.Fatal("incorrect latest sample")
	}
}
func TestSnapshotFiltersStaleAndFutureSamples(t *testing.T) {
	cache, _ := NewCache(10)
	now := time.Unix(100, 0)
	for _, sample := range []Sample{{Channel: "old", Unit: "K", Value: 1, Time: now.Add(-time.Minute)}, {Channel: "future", Unit: "K", Value: 1, Time: now.Add(time.Minute)}, {Channel: "current", Unit: "K", Value: 1, Time: now}} {
		if err := cache.Append(sample); err != nil {
			t.Fatal(err)
		}
	}
	snapshot := cache.Snapshot(now, 5*time.Second)
	if len(snapshot) != 1 || snapshot[0].Channel != "current" {
		t.Fatal("snapshot contained invalid age")
	}
}
func TestInvalidDataCannotReplaceLatestSample(t *testing.T) {
	cache, _ := NewCache(2)
	now := time.Unix(100, 0)
	sample := Sample{Channel: "battery", Unit: "fraction", Value: 0.8, Time: now}
	if err := cache.Append(sample); err != nil {
		t.Fatal(err)
	}
	sample.Value = math.NaN()
	if cache.Append(sample) == nil {
		t.Fatal("accepted NaN")
	}
	sample.Value = 0.7
	sample.Time = now.Add(-time.Second)
	if cache.Append(sample) == nil {
		t.Fatal("accepted reordered telemetry")
	}
	latest, _ := cache.Latest("battery")
	if latest.Value != 0.8 {
		t.Fatal("invalid observation altered latest sample")
	}
}
