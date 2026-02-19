package session

import (
	"testing"
	"time"
)

func TestIssue2041Nominal(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	store, _ := NewStore(time.Minute)
	a, _ := store.Create("operator", Controller, now)
	if store.RevokeOperator("operator") != 1 {
		t.Fatal("revocation count")
	}
	if _, ok := store.Lookup(a.Token, now); ok {
		t.Fatal("token remained active")
	}
}
