# Electrical power and battery reserve

Power allocation sorts loads by operational priority before consuming the available budget. Circuit protection uses instantaneous and accumulated overcurrent limits. Battery state integrates charge and discharge efficiency.

## Invariants

- Reserve is a closed fraction interval.
- Flight control precedes payload allocation.
- Trip reset requires a zero-current observation.
- Solar incidence cannot generate negative power.

## Verification

Run the relevant unit suite and include nominal, boundary and rejected inputs.
Check units at interfaces and preserve existing call contracts. Record operating
changes in the release notes when the behavior affects an operator or peer.

## Change review

Use the domain owner listed in CONTRIBUTING.md. A cross-subsystem change must
state the contract dependency and include its peer-side vector or test.

## Battery reserve warning

Classify reserve bands and distinguish charging from discharge margin.

`flight/src/power/battery.rs` exposes `reserve_band`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.
