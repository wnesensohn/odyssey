package health

import (
	"time"
)

func ClassifyProbe(probe Probe, now time.Time, maximumAge time.Duration) string {
	if maximumAge <= 0 || probe.Checked.IsZero() || probe.Checked.After(now) || now.Sub(probe.Checked) > maximumAge {
		return "stale"
	}
	if !probe.Healthy {
		return "degraded"
	}
	return "healthy"
}
