package telemetry

func SampleRate(history []Sample) float64 {
	if len(history) < 2 {
		return 0
	}
	elapsed := history[len(history)-1].Time.Sub(history[0].Time).Seconds()
	if elapsed <= 0 {
		return 0
	}
	return float64(len(history)-1) / elapsed
}
