package ui

import (
	"net/http/httptest"
	"odyssey.example.test/console/internal/telemetry"
	"strings"
	"testing"
)

func TestIssue2013Case2(t *testing.T) {
	server := testServer(t)
	_ = server.Cache.Append(telemetry.Sample{Channel: "pump", Unit: "kPa", Time: server.Now(), Quality: "invalid"})
	response := httptest.NewRecorder()
	server.Routes("../../web").ServeHTTP(response, httptest.NewRequest("GET", "/fragments/telemetry", nil))
	if !strings.Contains(response.Body.String(), "Invalid") {
		t.Fatal("missing operator context", response.Body.String())
	}
}
