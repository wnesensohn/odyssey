# Architecture

The flight library owns actuator state and simulated hardware. Communications
workers own framing, session reliability and topic routing. The console owns
operator identity, queued command requests and telemetry presentation.

## Data path

1. An operator requests a typed command through an HTMX form.
2. The Go service validates identity, bounds and expiry.
3. A frame crosses the ground-link session and passes checksum validation.
4. The flight controller checks interlocks before changing actuator state.
5. Observations return as telemetry with explicit units and quality.

## Isolation

Controller loops never block on browser connections. Queue capacities and
receive deadlines bound work. Failing observers are removed from the router.
Critical faults preserve a latched safe state until an explicit recovery.
