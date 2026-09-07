package ui

import (
	"net/http/httptest"
	"strings"
	"testing"
)

func TestSafetyControlsAdvertiseKeyboardAccess(t *testing.T) {
	s := testServer(t)
	w := httptest.NewRecorder()
	s.Routes("../../web").ServeHTTP(w, httptest.NewRequest("GET", "/", nil))
	for _, attribute := range []string{`accesskey="s"`, `accesskey="h"`, `aria-keyshortcuts="S"`} {
		if !strings.Contains(w.Body.String(), attribute) {
			t.Fatal("shortcut missing", attribute)
		}
	}
}
