package ui

import (
	"net/http/httptest"
	"strings"
	"testing"
)

func TestIssue2084Case0(t *testing.T) {
	server := testServer(t)
	response := httptest.NewRecorder()
	server.Routes("../../web").ServeHTTP(response, httptest.NewRequest("GET", "/", nil))
	if !strings.Contains(response.Body.String(), "aria-describedby=\"telemetry-help\"") {
		t.Fatal("missing operator context", response.Body.String())
	}
}
