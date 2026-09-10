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

## Heater duty budget

Integrate heater energy and reject requests exceeding a bounded interval budget.

`flight/src/thermal/thermal.rs` exposes `heater_energy_wh`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.

## Thermal sensor voting

Reject outliers before selecting redundant thermal sensor readings.

`flight/src/thermal/thermal.rs` exposes `voted_temperature`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.

## Thermal cooldown monitor

Detect stalled cooldown using bounded sample windows.

`flight/src/thermal/coolant.rs` exposes `cooldown_stalled`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.

## Extract heater energy accounting into a budget module

`heater_energy_wh` lives in `flight/src/thermal/budget.rs`. Existing callers retain the same public
contract. Regression tests cover its boundary behavior after the move.

## Acknowledged fault clearance

Prevent clearing active faults before their quiet interval. The public interface reports rejected inputs without mutating its
actuator or queue state. Regression tests cover the normal path and the limiting
case; operator procedures should handle both outcomes explicitly.

## Thermal settle window

The `thermal-settle-window` change remains under review. Regression tests cover its boundary
behavior. Before integration, compare its policy with the current development
branch and resolve the documented differences deliberately.
