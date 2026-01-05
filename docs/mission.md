# Mission phase sequencing

Commissioning completes into orbit hold. Transfer needs guidance, propulsion, communications and a power reserve. Science permits payload operations. Safe mode routes recovery through an explicit prerequisite check.

## Invariants

- No direct safe-to-transfer transition.
- A critical fault blocks normal requests.
- Count phase changes and elapsed time separately.
- Repeated requests for the current phase are idempotent.

## Verification

Run the relevant unit suite and include nominal, boundary and rejected inputs.
Check units at interfaces and preserve existing call contracts. Record operating
changes in the release notes when the behavior affects an operator or peer.

## Change review

Use the domain owner listed in CONTRIBUTING.md. A cross-subsystem change must
state the contract dependency and include its peer-side vector or test.
