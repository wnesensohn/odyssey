# Engine and feed control

The feed sequencer opens the inlet before enabling pump regulation. Ignition requires a ready pressure band and confirmed outlet movement. Chamber overtemperature locks the feed system and closes commanded valves.

## Invariants

- Engine safe -> priming -> igniting -> running -> cooling.
- Pump stopped -> starting -> regulating; trip latches zero duty.
- Valve contradictory switches are invalid, not a guessed position.
- Pressure observations use kPa and finite samples.

## Verification

Run the relevant unit suite and include nominal, boundary and rejected inputs.
Check units at interfaces and preserve existing call contracts. Record operating
changes in the release notes when the behavior affects an operator or peer.

## Change review

Use the domain owner listed in CONTRIBUTING.md. A cross-subsystem change must
state the contract dependency and include its peer-side vector or test.

## Pump pressure interlock

Reject stale feed-pressure observations before enabling ignition.

`flight/src/propulsion/pressure.rs` exposes `fresh_pressure_permits_ignition`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.

## Valve position debounce

Debounce switches without treating contradictory positions as valid.

`flight/src/propulsion/valve.rs` exposes `debounced_position`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.

## Pump startup cavitation

Detect low inlet pressure during pump startup and latch a clear trip reason.

`flight/src/propulsion/pump.rs` exposes `cavitation_margin_kpa`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.

## Propellant reserve estimate

Estimate usable propellant after reserve and temperature constraints.

`flight/src/propulsion/tank.rs` exposes `usable_propellant_kg`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.

## Engine shutdown purge

Track purge completion before returning the feed system to isolated state.

`flight/src/propulsion/feed.rs` exposes `purge_complete`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.

## Repeated valve command

The `repeated-valve-command` change remains under review. Regression tests cover its boundary
behavior. Before integration, compare its policy with the current development
branch and resolve the documented differences deliberately.
