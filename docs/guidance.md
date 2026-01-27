# Guidance, navigation and shaping

Navigation maintains Cartesian position and velocity in an inertial frame. Rate control limits vector torque. Shaping rate and acceleration limits prevent discontinuous panel commands.

## Invariants

- Propagate with bounded time steps.
- Normalize quaternions before applying rotations.
- Reject zero vector normalization.
- Use transfer calculations as planning inputs, not immediate actuator commands.

## Verification

Run the relevant unit suite and include nominal, boundary and rejected inputs.
Check units at interfaces and preserve existing call contracts. Record operating
changes in the release notes when the behavior affects an operator or peer.

## Change review

Use the domain owner listed in CONTRIBUTING.md. A cross-subsystem change must
state the contract dependency and include its peer-side vector or test.

## Quaternion error budget

Calculate shortest-path orientation error and cap requested angular correction.

`flight/src/guidance/attitude.rs` exposes `orientation_error_rad`. Values use SI units, except
pressure in kPa, energy in Wh and mission time in milliseconds. Invalid numeric
inputs fail before the output is applied to an actuator. Callers must handle the
returned result; an error never authorizes an actuator transition.
