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

## Payload load shedding

Explain unsupplied payload loads and preserve flight-control allocation.

`flight/src/power/power.rs` exposes `unsupplied_loads`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.

## Battery charge temperature

Apply a temperature-dependent charge acceptance envelope.

`flight/src/power/battery.rs` exposes `charge_acceptance_fraction`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.

## Solar array incidence

Report eclipse and grazing-incidence cases independently of electrical failures.

`flight/src/power/power.rs` exposes `illumination_state`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.

## Power restore order

Restore shed circuits in explicit priority order with reserve checks.

`flight/src/power/power.rs` exposes `restore_order`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.

## Duplicate load identifiers

Reject ambiguous duplicate loads before bus allocation. The public interface reports rejected inputs without mutating its
actuator or queue state. Regression tests cover the normal path and the limiting
case; operator procedures should handle both outcomes explicitly.
