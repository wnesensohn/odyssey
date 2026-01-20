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
