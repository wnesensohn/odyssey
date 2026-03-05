use crate::attitude::Vector3;
use crate::{finite_in_range, ControlError};

pub const EARTH_MU: f64 = 3.986_004_418e14;

#[derive(Debug, Clone, Copy)]
pub struct OrbitalState {
    pub position_m: Vector3,
    pub velocity_mps: Vector3,
    pub timestamp_ms: u64,
}

impl OrbitalState {
    pub fn specific_energy(&self) -> Result<f64, ControlError> {
        let radius = self.position_m.norm();
        finite_in_range(radius, 6_000_000.0, 1e9)?;
        let speed = self.velocity_mps.norm();
        finite_in_range(speed, 0.0, 100_000.0)?;
        Ok(0.5 * speed * speed - EARTH_MU / radius)
    }

    pub fn angular_momentum(&self) -> Vector3 {
        self.position_m.cross(self.velocity_mps)
    }

    pub fn propagate(&self, dt_s: f64) -> Result<Self, ControlError> {
        finite_in_range(dt_s, 0.0, 10.0)?;
        self.specific_energy()?;
        let radius = self.position_m.norm();
        let acceleration = self.position_m.scaled(-EARTH_MU / radius.powi(3));
        Ok(Self {
            position_m: Vector3 {
                x: self.position_m.x
                    + self.velocity_mps.x * dt_s
                    + 0.5 * acceleration.x * dt_s * dt_s,
                y: self.position_m.y
                    + self.velocity_mps.y * dt_s
                    + 0.5 * acceleration.y * dt_s * dt_s,
                z: self.position_m.z
                    + self.velocity_mps.z * dt_s
                    + 0.5 * acceleration.z * dt_s * dt_s,
            },
            velocity_mps: Vector3 {
                x: self.velocity_mps.x + acceleration.x * dt_s,
                y: self.velocity_mps.y + acceleration.y * dt_s,
                z: self.velocity_mps.z + acceleration.z * dt_s,
            },
            timestamp_ms: self
                .timestamp_ms
                .saturating_add((dt_s * 1000.0).round() as u64),
        })
    }
}

pub fn horizon_angle(radius_m: f64, body_radius_m: f64) -> Result<f64, ControlError> {
    finite_in_range(body_radius_m, 1.0, 1e8)?;
    finite_in_range(radius_m, body_radius_m, 1e10)?;
    Ok((body_radius_m / radius_m).acos())
}

pub fn propagation_interval_ms(previous_ms: u64, next_ms: u64) -> Result<u64, crate::ControlError> {
    let interval = next_ms
        .checked_sub(previous_ms)
        .ok_or(crate::ControlError::InvalidTransition)?;
    if interval > 10_000 {
        return Err(crate::ControlError::OutOfRange);
    }
    Ok(interval)
}
