package ui

import (
	"net/http/httptest"
	"odyssey.example.test/console/internal/telemetry"
	"strings"
	"testing"
)

func TestIssue2084Case1(t *testing.T) {
	server := testServer(t)
	_ = server.Cache.Append(telemetry.Sample{Channel: "pump", Unit: "kPa", Time: server.Now(), Quality: "good"})
	response := httptest.NewRecorder()
	server.Routes("../../web").ServeHTTP(response, httptest.NewRequest("GET", "/", nil))
	if !strings.Contains(response.Body.String(), "scope=\"row\"") {
		t.Fatal("missing operator context", response.Body.String())
	}
}
