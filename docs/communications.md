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

## Bounded ground link acceptors

Limit simultaneous gateway workers and shut them down with their listener.

`comms/src/transport/odyssey_gateway.erl` exposes `admit_worker/2`. Link times are monotonic
milliseconds; sequence fields use unsigned network byte order. Rejection is
reported to the caller and must not be converted into an acknowledged delivery.

## Wire payload inspection

Expose header/payload diagnostics without accepting invalid checksums.

`comms/src/transport/odyssey_frame.erl` exposes `inspect/1`. Link times are monotonic
milliseconds; sequence fields use unsigned network byte order. Rejection is
reported to the caller and must not be converted into an acknowledged delivery.

## Acknowledgement timeouts

Track acknowledgement latency and cap retransmission jitter.

`comms/src/transport/odyssey_window.erl` exposes `ack_latency/2`. Link times are monotonic
milliseconds; sequence fields use unsigned network byte order. Rejection is
reported to the caller and must not be converted into an acknowledged delivery.

## Wire envelope version four

Widen frame sequence numbers to 32 bits and document the incompatible envelope.

The mission console renders the change through local Go templates and HTMX
fragments. Controller authentication and flight interlocks remain authoritative.
Labels are escaped by the template engine; no third-party asset request is needed.

## Session backpressure

Report pending-window backpressure to command callers.

`comms/src/transport/odyssey_session.erl` exposes `pending_state/2`. Link times are monotonic
milliseconds; sequence fields use unsigned network byte order. Rejection is
reported to the caller and must not be converted into an acknowledged delivery.

## Protocol peer capabilities

Expose compatible peer capabilities without changing mandatory envelope fields.

`comms/src/transport/odyssey_frame.erl` exposes `capabilities/1`. Link times are monotonic
milliseconds; sequence fields use unsigned network byte order. Rejection is
reported to the caller and must not be converted into an acknowledged delivery.

## Retry ceiling

Cap the retry policy even when callers request an excessive budget. The public interface reports rejected inputs without mutating its
actuator or queue state. Regression tests cover the normal path and the limiting
case; operator procedures should handle both outcomes explicitly.
