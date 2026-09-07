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

## Command audit records

Record accepted/rejected command identities and validation outcomes.

`console/internal/commands` exposes `AuditOutcome` to console adapters.
All timestamps are UTC, and operators are identified by their controller session.
Rejected requests do not authorize a flight command. Queue limits are independent
of display polling intervals.

## Accessible telemetry controls

Add accessible table controls and stable fragment focus behavior.

The mission console renders the change through local Go templates and HTMX
fragments. Controller authentication and flight interlocks remain authoritative.
Labels are escaped by the template engine; no third-party asset request is needed.

## Health freshness budget

Classify degraded health separately from missing or stale observations.

`console/internal/health` exposes `ClassifyProbe` to console adapters.
All timestamps are UTC, and operators are identified by their controller session.
Rejected requests do not authorize a flight command. Queue limits are independent
of display polling intervals.

## Idempotent command requests

Return the prior result for a valid duplicate operator request.

`console/internal/commands` exposes `RequestFingerprint` to console adapters.
All timestamps are UTC, and operators are identified by their controller session.
Rejected requests do not authorize a flight command. Queue limits are independent
of display polling intervals.

## Mission phase status

Display phase transitions and their unmet prerequisite reasons.

The mission console renders the change through local Go templates and HTMX
fragments. Controller authentication and flight interlocks remain authoritative.
Labels are escaped by the template engine; no third-party asset request is needed.

## Telemetry rate accounting

Measure sample arrival rates without changing engineering-unit values.

`console/internal/telemetry` exposes `SampleRate` to console adapters.
All timestamps are UTC, and operators are identified by their controller session.
Rejected requests do not authorize a flight command. Queue limits are independent
of display polling intervals.

## Command result stream

Publish command execution outcomes through a bounded event stream.

`console/internal/events` exposes `CommandOutcome` to console adapters.
All timestamps are UTC, and operators are identified by their controller session.
Rejected requests do not authorize a flight command. Queue limits are independent
of display polling intervals.

## Operator event filters

Filter the event journal without dropping severity or time context.

The mission console renders the change through local Go templates and HTMX
fragments. Controller authentication and flight interlocks remain authoritative.
Labels are escaped by the template engine; no third-party asset request is needed.

## Telemetry retention metrics

Report dropped sample counts and retained-history bounds.

`console/internal/telemetry` exposes `RetentionBounds` to console adapters.
All timestamps are UTC, and operators are identified by their controller session.
Rejected requests do not authorize a flight command. Queue limits are independent
of display polling intervals.

## Keep command timing policy beside validation

`ValidatedLifetime` lives in `console/internal/commands/service.go`. Existing callers retain the same public
contract. Regression tests cover its boundary behavior after the move.

## Group command outcomes with journal events

`CommandOutcome` lives in `console/internal/events/journal.go`. Existing callers retain the same public
contract. Regression tests cover its boundary behavior after the move.

## Place latest observations beside history access

`Latest` lives in `console/internal/telemetry/cache.go`. Existing callers retain the same public
contract. Regression tests cover its boundary behavior after the move.

## Persistent channel retention

Preserve channel retention limits across subsequent observations. The public interface reports rejected inputs without mutating its
actuator or queue state. Regression tests cover the normal path and the limiting
case; operator procedures should handle both outcomes explicitly.

## Operator identity bounds

Reject operator identities longer than 128 bytes. The public interface reports rejected inputs without mutating its
actuator or queue state. Regression tests cover the normal path and the limiting
case; operator procedures should handle both outcomes explicitly.

## Retained event filter

Retain event severity filters during HTMX polling. The public interface reports rejected inputs without mutating its
actuator or queue state. Regression tests cover the normal path and the limiting
case; operator procedures should handle both outcomes explicitly.

## Keyboard command shortcuts

The `keyboard-command-shortcuts` change remains under review. Regression tests cover its boundary
behavior. Before integration, compare its policy with the current development
branch and resolve the documented differences deliberately.
