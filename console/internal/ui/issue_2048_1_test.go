package ui

import (
	"net/http/httptest"
	"strings"
	"testing"
)

func TestIssue2048Case1(t *testing.T) {
	server := testServer(t)
	response := httptest.NewRecorder()
	server.Routes("../../web").ServeHTTP(response, httptest.NewRequest("GET", "/", nil))
	if !strings.Contains(response.Body.String(), "kind\" value=\"arm-engine") {
		t.Fatal("missing operator context", response.Body.String())
	}
}
