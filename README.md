# ODYSSEY

**Orbital Dynamics, Sapience, Shaping, Exploration & Yonder**

ODYSSEY coordinates a simulated orbital spacecraft, its flight subsystems,
ground-link transport and operator console. All engineering quantities use
documented SI units. Hardware interfaces have deterministic software adapters.

## System layout

- `flight/`: Rust controllers, state machines, plant models and protocol codec.
- `comms/`: Erlang OTP supervision, framing, sessions and telemetry routing.
- `console/`: Go backend and locally served HTMX operator interface.
- `contracts/`: transport and subsystem interface definitions.
- `config/`: vehicle limits and operational settings.
- `sim/`: deterministic mission and fault-injection inputs.
- `docs/`: design notes, operating procedures and subsystem contracts.

## Build and verify

Use Rust 1.75 or newer, Go 1.22 or newer, Erlang/OTP 25 and erlfmt.
The code uses standard libraries and OTP; HTMX 2.0.8 is vendored with its license.

```sh
python3 tools/check.py
cargo run --manifest-path flight/Cargo.toml --bin flight-sim
cd console
go run ./cmd/mission-console -web web -listen 127.0.0.1:8080
```

The console exposes read-only telemetry and health without an operator session.
Command submission requires an active controller session. Production identity
provisioning is handled by the deployment adapter, not a hardcoded HTTP password.

## Development workflow

`main` contains production releases; `develop` integrates accepted features.
Features use `feature/####-short-name`. Release stabilization uses `release/X.Y.Z`.
Production fixes use `bugfix/####-short-name` and are integrated into production
and development. Annotated tags identify released manifests and protocol versions.
See [CONTRIBUTING.md](CONTRIBUTING.md) for subsystem ownership and review rules.
