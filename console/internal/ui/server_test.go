package ui

import (
	"net/http"
	"net/http/httptest"
	"net/url"
	"odyssey.example.test/console/internal/session"
	"odyssey.example.test/console/internal/telemetry"
	"strings"
	"testing"
	"time"
)

func testServer(t *testing.T) *Server {
	t.Helper()
	server, err := NewServer("../../web")
	if err != nil {
		t.Fatal(err)
	}
	server.Now = func() time.Time { return time.Unix(100, 0) }
	return server
}
func TestOverviewAndHTMXFragmentsRenderWithoutNetworkAssets(t *testing.T) {
	server := testServer(t)
	for _, path := range []string{"/", "/fragments/telemetry", "/fragments/events"} {
		response := httptest.NewRecorder()
		server.Routes("../../web").ServeHTTP(response, httptest.NewRequest("GET", path, nil))
		if response.Code != http.StatusOK {
			t.Fatalf("%s returned %d", path, response.Code)
		}
		if !strings.Contains(response.Header().Get("Content-Type"), "text/html") {
			t.Fatal("fragment has incorrect content type")
		}
	}
}
func TestTelemetryTemplatesEscapeUntrustedLabels(t *testing.T) {
	server := testServer(t)
	_ = server.Cache.Append(telemetry.Sample{Channel: "<script>alert(1)</script>", Unit: "K", Value: 290, Time: server.Now(), Quality: "good"})
	response := httptest.NewRecorder()
	server.Routes("../../web").ServeHTTP(response, httptest.NewRequest("GET", "/fragments/telemetry", nil))
	if strings.Contains(response.Body.String(), "<script>") {
		t.Fatal("channel label was not escaped")
	}
}
func TestCommandSubmissionRequiresControllerSession(t *testing.T) {
	server := testServer(t)
	form := url.Values{"kind": {"set-throttle"}, "value": {"0.5"}}.Encode()
	request := httptest.NewRequest("POST", "/commands", strings.NewReader(form))
	request.Header.Set("Content-Type", "application/x-www-form-urlencoded")
	response := httptest.NewRecorder()
	server.Routes("../../web").ServeHTTP(response, request)
	if response.Code != http.StatusUnauthorized {
		t.Fatal("anonymous command was accepted")
	}
	identity, _ := server.Sessions.Create("flight-operator", session.Controller, server.Now())
	request = httptest.NewRequest("POST", "/commands", strings.NewReader(form))
	request.Header.Set("Content-Type", "application/x-www-form-urlencoded")
	request.AddCookie(&http.Cookie{Name: "odyssey-session", Value: identity.Token})
	response = httptest.NewRecorder()
	server.Routes("../../web").ServeHTTP(response, request)
	if response.Code != http.StatusAccepted || server.Commands.Pending() != 1 {
		t.Fatal("controller command was rejected")
	}
}
