# Build and qualification

ODYSSEY uses standard libraries and OTP. HTMX 2.0.8 is vendored with its MIT
license, so rendering the console requires no asset download.

## Toolchain

The qualified Linux toolchain is Rust 1.85.1 with rustfmt, Go 1.27.1 and
Erlang/OTP 25. The source also targets Rust 1.75+ and Go 1.22+.
Install Python 3 and a C linker for the Rust test executables. On Debian, install
`build-essential erlang-base erlang-dev erlang-eunit erlang-syntax-tools`.
The formatter uses erlfmt 1.8.0; compile its modules and set `ERLFMT_EBIN` to their
output directory before running `escript tools/format_erlang.escript FILE...`.

Run `python3 tools/check.py` at the repository root. It checks Rust and Go
formatting, builds both packages, compiles all grouped Erlang source and executes
the EUnit suite. Compiler caches and BEAM outputs remain outside tracked files.

## Operator checks

Run `cargo run --manifest-path flight/Cargo.toml --bin flight-sim` for the nominal
engine sequence. At wire version 4, its `frame=` output can be passed to
`go run ./cmd/frame-inspect HEX` from `console/` to verify codec compatibility.
Start the console with `go run ./cmd/mission-console -web web` from that directory.
The mission-console tests render the HTMX fragments and exercise session checks.

The software models a spacecraft and deterministic adapters. Physical hardware
interfaces, mission certification and deployment authentication are supplied by
the integration environment.
