package session

import (
	"testing"
	"time"
)

func TestSessionsExpireAndRevokeWithoutTokenReuse(t *testing.T) {
	now := time.Unix(100, 0)
	store, _ := NewStore(time.Minute)
	first, _ := store.Create("Mara Stein", Controller, now)
	second, _ := store.Create("Sofia Alvarez", Observer, now)
	if first.Token == second.Token || len(first.Token) != 64 {
		t.Fatal("session tokens are not independent")
	}
	if identity, ok := store.Lookup(first.Token, now); !ok || identity.Role != Controller {
		t.Fatal("created session missing")
	}
	store.Revoke(first.Token)
	if _, ok := store.Lookup(first.Token, now); ok {
		t.Fatal("revoked session remained active")
	}
	if _, ok := store.Lookup(second.Token, now.Add(time.Minute)); ok {
		t.Fatal("session accepted at expiry")
	}
}
func TestSessionCreationRejectsUnsupportedRoles(t *testing.T) {
	store, _ := NewStore(time.Minute)
	if _, err := store.Create("", Controller, time.Now()); err == nil {
		t.Fatal("accepted empty operator")
	}
	if _, err := store.Create("operator", "administrator", time.Now()); err == nil {
		t.Fatal("accepted unsupported role")
	}
}

func TestSessionCreationRejectsUnsetClock(t *testing.T) {
	store, _ := NewStore(time.Minute)
	if _, err := store.Create("operator", Controller, time.Time{}); err == nil {
		t.Fatal("unset clock accepted")
	}
	if len(store.entries) != 0 {
		t.Fatal("rejected session stored")
	}
}
