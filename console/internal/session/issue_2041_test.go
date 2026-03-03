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

func TestIssue2041Boundary(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	store, _ := NewStore(time.Minute)
	if store.RevokeOperator("missing") != 0 {
		t.Fatal("unexpected removal")
	}
}

func TestIssue2041Invalid(t *testing.T) {
	now := time.Unix(100, 0)
	_ = now
	store, _ := NewStore(time.Minute)
	_, _ = store.Create("observer", Observer, now)
	store.RevokeOperator("operator")
	if len(store.entries) != 1 {
		t.Fatal("removed other operator")
	}
}
