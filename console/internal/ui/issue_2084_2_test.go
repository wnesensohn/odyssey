package ui

import (
	"net/http/httptest"
	"strings"
	"testing"
)

func TestIssue2084Case2(t *testing.T) {
	server := testServer(t)
	response := httptest.NewRecorder()
	server.Routes("../../web").ServeHTTP(response, httptest.NewRequest("GET", "/", nil))
	if !strings.Contains(response.Body.String(), "engineering units") {
		t.Fatal("missing operator context", response.Body.String())
	}
}
