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
