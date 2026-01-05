# Build and verification

The workspace has no downloaded code dependencies except the pinned local HTMX asset. Compiler outputs stay outside tracked source. Each language has a standard-library test suite.

## Invariants

- cargo fmt and cargo test.
- gofmt and go test.
- erlc -Werror and EUnit.
- Template rendering and transport vectors.

## Verification

Run the relevant unit suite and include nominal, boundary and rejected inputs.
Check units at interfaces and preserve existing call contracts. Record operating
changes in the release notes when the behavior affects an operator or peer.

## Change review

Use the domain owner listed in CONTRIBUTING.md. A cross-subsystem change must
state the contract dependency and include its peer-side vector or test.
