# Release notes

## Unreleased

- Flight, communications and mission-console baseline ready for integration.
- Deterministic controller tests and shared frame checksum vector established.

Release entries describe observable interface or operating changes.

## 3.0.0

- Reject stale feed-pressure observations before enabling ignition.

## 3.1.0

- Bound retries and report frames exhausted by the retransmission window.
- Classify reserve bands and distinguish charging from discharge margin.
- Reject commands whose lifetime or time ordering cannot be represented safely.
- Render invalid and degraded samples with clear status labels.

## 3.2.0

- Integrate heater energy and reject requests exceeding a bounded interval budget.
- Calculate shortest-path orientation error and cap requested angular correction.
- Support per-channel retention while preserving ordered snapshots.
- Define compatible command/ack/telemetry kind validation in all peer codecs.

## 3.2.1

- Reject commands at their exact expiry instant.

## 3.3.0

- Explain unsupplied payload loads and preserve flight-control allocation.
- Invalidate controller tokens after role changes and record revocation cause.
- Add an explicit operator confirmation step to engine-arm requests.
- Reject outliers before selecting redundant thermal sensor readings.

## 3.4.0

- Report valid propagation intervals and reject nonfinite derived state.
- Report journal gaps to observers instead of silently hiding dropped entries.
- Classify duplicate and stale sequence numbers in the negotiated window.
- Detect low inlet pressure during pump startup and latch a clear trip reason.

## 3.5.0

- Limit simultaneous gateway workers and shut them down with their listener.
- Apply a temperature-dependent charge acceptance envelope.
- Record accepted/rejected command identities and validation outcomes.
- Add accessible table controls and stable fragment focus behavior.

## 3.5.1

- Return an error for malformed retransmission reservations.

## 4.0.0

- Preserve fault history when an operator clears an active condition.
- Bound accepted estimator innovations and report rejection statistics.
- Measure sample arrival rates without changing engineering-unit values.
- Widen frame sequence numbers to 32 bits and document the incompatible envelope.
