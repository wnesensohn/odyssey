# Operator console

The Go backend validates controller identity and command timing before queueing requests. Templates escape telemetry labels. HTMX refreshes local fragments without replacing the complete page.

## Invariants

- Observe telemetry without issuing commands.
- Command submission requires a controller session.
- Use bounded telemetry history and event journals.
- Expired commands drain without execution.

## Verification

Run the relevant unit suite and include nominal, boundary and rejected inputs.
Check units at interfaces and preserve existing call contracts. Record operating
changes in the release notes when the behavior affects an operator or peer.

## Change review

Use the domain owner listed in CONTRIBUTING.md. A cross-subsystem change must
state the contract dependency and include its peer-side vector or test.
