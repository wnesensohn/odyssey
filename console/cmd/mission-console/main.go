package main

import (
	"flag"
	"log"
	"net/http"
	"odyssey.example.test/console/internal/health"
	"odyssey.example.test/console/internal/telemetry"
	"odyssey.example.test/console/internal/ui"
	"time"
)

func main() {
	address := flag.String("listen", "127.0.0.1:8080", "HTTP listen address")
	web := flag.String("web", "web", "template and static asset directory")
	flag.Parse()
	service, err := ui.NewServer(*web)
	if err != nil {
		log.Fatal(err)
	}
	now := time.Now()
	for _, sample := range []telemetry.Sample{
		{Channel: "propulsion.feed-pressure", Unit: "kPa", Value: 300, Time: now, Quality: "good"},
		{Channel: "power.battery-charge", Unit: "fraction", Value: 0.84, Time: now, Quality: "good"},
		{Channel: "thermal.avionics", Unit: "K", Value: 294.15, Time: now, Quality: "good"},
	} {
		if err := service.Cache.Append(sample); err != nil {
			log.Fatal(err)
		}
	}
	_ = service.Health.Update(health.Probe{Name: "console", Healthy: true, Detail: "ready", Checked: now})
	server := &http.Server{Addr: *address, Handler: service.Routes(*web), ReadHeaderTimeout: 5 * time.Second, ReadTimeout: 10 * time.Second, WriteTimeout: 10 * time.Second, IdleTimeout: 30 * time.Second}
	log.Fatal(server.ListenAndServe())
}
