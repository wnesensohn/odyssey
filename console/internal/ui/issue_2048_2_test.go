package ui

import (
	"net/http/httptest"
	"strings"
	"testing"
)

func TestIssue2048Case2(t *testing.T) {
	server := testServer(t)
	response := httptest.NewRecorder()
	server.Routes("../../web").ServeHTTP(response, httptest.NewRequest("GET", "/", nil))
	if !strings.Contains(response.Body.String(), "Verify the feed interlocks") {
		t.Fatal("missing operator context", response.Body.String())
	}
}
