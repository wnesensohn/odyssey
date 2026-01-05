package ui

import (
	"encoding/json"
	"errors"
	"html/template"
	"io"
	"net/http"
	"path/filepath"
	"strconv"
	"time"

	"odyssey.example.test/console/internal/commands"
	"odyssey.example.test/console/internal/events"
	"odyssey.example.test/console/internal/health"
	"odyssey.example.test/console/internal/session"
	"odyssey.example.test/console/internal/telemetry"
)

type Server struct {
	Templates *template.Template
	Cache     *telemetry.Cache
	Commands  *commands.Service
	Journal   *events.Journal
	Health    *health.Registry
	Sessions  *session.Store
	Now       func() time.Time
}
type Dashboard struct {
	Samples []telemetry.Sample
	Events  []events.Event
	Pending int
}

func NewServer(webDirectory string) (*Server, error) {
	templates, err := template.ParseGlob(filepath.Join(webDirectory, "templates", "*.html"))
	if err != nil {
		return nil, err
	}
	cache, _ := telemetry.NewCache(120)
	commandService, _ := commands.NewService(32)
	journal, _ := events.NewJournal(200)
	sessions, _ := session.NewStore(30 * time.Minute)
	return &Server{Templates: templates, Cache: cache, Commands: commandService, Journal: journal, Health: health.NewRegistry(), Sessions: sessions, Now: time.Now}, nil
}

func (s *Server) Routes(webDirectory string) http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /", s.overview)
	mux.HandleFunc("GET /fragments/telemetry", s.telemetryFragment)
	mux.HandleFunc("GET /fragments/events", s.eventsFragment)
	mux.HandleFunc("GET /api/telemetry", s.telemetryJSON)
	mux.HandleFunc("GET /healthz", s.healthJSON)
	mux.HandleFunc("POST /commands", s.submitCommand)
	mux.Handle("GET /static/", http.StripPrefix("/static/", http.FileServer(http.Dir(filepath.Join(webDirectory, "static")))))
	return mux
}

func (s *Server) render(w http.ResponseWriter, name string, data any) {
	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	if err := s.Templates.ExecuteTemplate(w, name, data); err != nil {
		http.Error(w, "template rendering failed", http.StatusInternalServerError)
	}
}
func (s *Server) overview(w http.ResponseWriter, r *http.Request) {
	s.render(w, "overview", Dashboard{Samples: s.Cache.Snapshot(s.Now(), 5*time.Second), Events: s.Journal.Since(0), Pending: s.Commands.Pending()})
}
func (s *Server) telemetryFragment(w http.ResponseWriter, r *http.Request) {
	s.render(w, "telemetry", s.Cache.Snapshot(s.Now(), 5*time.Second))
}
func (s *Server) eventsFragment(w http.ResponseWriter, r *http.Request) {
	s.render(w, "events", s.Journal.Since(0))
}
func (s *Server) telemetryJSON(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	_ = json.NewEncoder(w).Encode(s.Cache.Snapshot(s.Now(), 5*time.Second))
}
func (s *Server) healthJSON(w http.ResponseWriter, r *http.Request) {
	probes, healthy := s.Health.Snapshot(s.Now(), 10*time.Second)
	w.Header().Set("Content-Type", "application/json")
	if !healthy {
		w.WriteHeader(http.StatusServiceUnavailable)
	}
	_ = json.NewEncoder(w).Encode(probes)
}

func commandFromForm(r *http.Request, operator string, now time.Time) (commands.Command, error) {
	if err := r.ParseForm(); err != nil {
		return commands.Command{}, err
	}
	value, err := strconv.ParseFloat(r.Form.Get("value"), 64)
	if err != nil {
		return commands.Command{}, errors.New("invalid numeric command value")
	}
	return commands.Command{Kind: commands.Kind(r.Form.Get("kind")), Value: value, Operator: operator, Issued: now, Expires: now.Add(5 * time.Second)}, nil
}

func (s *Server) submitCommand(w http.ResponseWriter, r *http.Request) {
	r.Body = http.MaxBytesReader(w, r.Body, 4096)
	cookie, err := r.Cookie("odyssey-session")
	if err != nil {
		http.Error(w, "operator session required", http.StatusUnauthorized)
		return
	}
	identity, ok := s.Sessions.Lookup(cookie.Value, s.Now())
	if !ok || identity.Role != session.Controller {
		http.Error(w, "controller role required", http.StatusForbidden)
		return
	}
	command, err := commandFromForm(r, identity.Operator, s.Now())
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	accepted, err := s.Commands.Submit(command, s.Now())
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	_, _ = s.Journal.Append(events.Event{Time: s.Now(), Area: "commands", Severity: "info", Message: "Accepted " + string(accepted.Kind)})
	w.Header().Set("Content-Type", "text/plain; charset=utf-8")
	w.WriteHeader(http.StatusAccepted)
	_, _ = io.WriteString(w, "Command accepted")
}
