package session

func (s *Store) RevokeOperator(operator string) int {
	s.mu.Lock()
	defer s.mu.Unlock()
	count := 0
	for token, value := range s.entries {
		if value.Operator == operator {
			delete(s.entries, token)
			count++
		}
	}
	return count
}
