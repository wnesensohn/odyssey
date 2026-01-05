//! Flight control interfaces use SI units and bounded, explicit state transitions.
pub mod propulsion;
pub use propulsion::{engine, pressure, pump, valve};
pub mod guidance;
pub use guidance::{attitude, filter, navigation};
pub mod power;
pub use power::battery;
pub mod avionics;
pub mod thermal;
pub use avionics::{actuator, command, fault, protocol, sensor, telemetry, time, watchdog};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlError {
    InvalidSample,
    OutOfRange,
    StaleSample,
    InterlockOpen,
    InvalidTransition,
    CapacityExceeded,
    InvalidFrame,
}

pub fn finite_in_range(value: f64, low: f64, high: f64) -> Result<f64, ControlError> {
    if !value.is_finite() || !low.is_finite() || !high.is_finite() {
        return Err(ControlError::InvalidSample);
    }
    if low > high || value < low || value > high {
        return Err(ControlError::OutOfRange);
    }
    Ok(value)
}
