
## Watchdog recovery quorum

Require healthy task observations before clearing repeated watchdog faults.

`flight/src/avionics/watchdog.rs` exposes `recovery_quorum`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.
