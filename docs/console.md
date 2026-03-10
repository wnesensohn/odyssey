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

## Command lifetime validation

Reject commands whose lifetime or time ordering cannot be represented safely.

`console/internal/commands` exposes `ValidatedLifetime` to console adapters.
All timestamps are UTC, and operators are identified by their controller session.
Rejected requests do not authorize a flight command. Queue limits are independent
of display polling intervals.

## Telemetry quality display

Render invalid and degraded samples with clear status labels.

The mission console renders the change through local Go templates and HTMX
fragments. Controller authentication and flight interlocks remain authoritative.
Labels are escaped by the template engine; no third-party asset request is needed.

## Telemetry channel retention

Support per-channel retention while preserving ordered snapshots.

`console/internal/telemetry` exposes `RetainChannel` to console adapters.
All timestamps are UTC, and operators are identified by their controller session.
Rejected requests do not authorize a flight command. Queue limits are independent
of display polling intervals.

## Operator session revocation

Invalidate controller tokens after role changes and record revocation cause.

`console/internal/session` exposes `RevokeOperator` to console adapters.
All timestamps are UTC, and operators are identified by their controller session.
Rejected requests do not authorize a flight command. Queue limits are independent
of display polling intervals.

## Engine command confirmation

Add an explicit operator confirmation step to engine-arm requests.

The mission console renders the change through local Go templates and HTMX
fragments. Controller authentication and flight interlocks remain authoritative.
Labels are escaped by the template engine; no third-party asset request is needed.

## Event journal gap reporting

Report journal gaps to observers instead of silently hiding dropped entries.

`console/internal/events` exposes `SinceWithGap` to console adapters.
All timestamps are UTC, and operators are identified by their controller session.
Rejected requests do not authorize a flight command. Queue limits are independent
of display polling intervals.
