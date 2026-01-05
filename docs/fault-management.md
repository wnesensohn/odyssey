# Fault handling and recovery

Fault severity can escalate but is not automatically downgraded by later reports. Acknowledgement records operator awareness without clearing critical state. Recovery requires restored prerequisites.

## Invariants

- Critical faults force safe mode.
- Watchdog expiry counts once per arming.
- Record first and last observation times.
- Fault resets are explicit transitions.

## Verification

Run the relevant unit suite and include nominal, boundary and rejected inputs.
Check units at interfaces and preserve existing call contracts. Record operating
changes in the release notes when the behavior affects an operator or peer.

## Change review

Use the domain owner listed in CONTRIBUTING.md. A cross-subsystem change must
state the contract dependency and include its peer-side vector or test.
