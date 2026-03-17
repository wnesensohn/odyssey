package events

func (j *Journal) SinceWithGap(id uint64) ([]Event, bool) {
	j.mu.RLock()
	defer j.mu.RUnlock()
	gap := len(j.entries) > 0 && id < j.entries[0].ID-1
	result := []Event{}
	for _, event := range j.entries {
		if event.ID > id {
			result = append(result, event)
		}
	}
	return result, gap
}
