# Ground-link communications

OTP workers isolate telemetry routing from ground-link sessions. Frames carry checksums and bounded payloads. A pending window correlates acknowledgements with transmitted sequence numbers.

## Invariants

- Use passive TCP and finite receive timeouts.
- Keep stream tails until a complete frame exists.
- Invalid checksums never reach a command handler.
- Monitor subscribers and remove dead processes.

## Verification

Run the relevant unit suite and include nominal, boundary and rejected inputs.
Check units at interfaces and preserve existing call contracts. Record operating
changes in the release notes when the behavior affects an operator or peer.

## Change review

Use the domain owner listed in CONTRIBUTING.md. A cross-subsystem change must
state the contract dependency and include its peer-side vector or test.

## Ground link retry window

Bound retries and report frames exhausted by the retransmission window.

`comms/src/transport/odyssey_window.erl` exposes `retry_budget/2`. Link times are monotonic
milliseconds; sequence fields use unsigned network byte order. Rejection is
reported to the caller and must not be converted into an acknowledged delivery.

## Frame kind validation

Define compatible command/ack/telemetry kind validation in all peer codecs.

`comms/src/transport/odyssey_frame.erl` exposes `valid_kind/1`. Link times are monotonic
milliseconds; sequence fields use unsigned network byte order. Rejection is
reported to the caller and must not be converted into an acknowledged delivery.

## Heartbeat link recovery

Require consecutive healthy heartbeat observations before recovery.

`comms/src/monitoring/odyssey_heartbeat.erl` exposes `recovery_ready/2`. Link times are monotonic
milliseconds; sequence fields use unsigned network byte order. Rejection is
reported to the caller and must not be converted into an acknowledged delivery.

## Sequence replay detection

Classify duplicate and stale sequence numbers in the negotiated window.

`comms/src/transport/odyssey_frame.erl` exposes `sequence_class/3`. Link times are monotonic
milliseconds; sequence fields use unsigned network byte order. Rejection is
reported to the caller and must not be converted into an acknowledged delivery.
