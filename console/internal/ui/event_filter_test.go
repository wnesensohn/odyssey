package ui

import (
	"net/http/httptest"
	"odyssey.example.test/console/internal/events"
	"strings"
	"testing"
)

func TestSeverityFilterDoesNotLeakOtherEvents(t *testing.T) {
	s := testServer(t)
	for _, severity := range []string{"info", "warning"} {
		_, _ = s.Journal.Append(events.Event{Time: s.Now(), Area: "pump", Severity: severity, Message: severity + " marker"})
	}
	w := httptest.NewRecorder()
	s.Routes("../../web").ServeHTTP(w, httptest.NewRequest("GET", "/fragments/events?severity=warning", nil))
	if strings.Contains(w.Body.String(), "info marker") || !strings.Contains(w.Body.String(), "warning marker") {
		t.Fatal(w.Body.String())
	}
}

func TestPollingIncludesCurrentSeveritySelection(t *testing.T) {
	s := testServer(t)
	w := httptest.NewRecorder()
	s.Routes("../../web").ServeHTTP(w, httptest.NewRequest("GET", "/", nil))
	if !strings.Contains(w.Body.String(), `hx-include="#event-severity"`) {
		t.Fatal("filter omitted from polling")
	}
}
