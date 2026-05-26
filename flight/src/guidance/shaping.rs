use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone)]
pub struct PanelController {
    pub angle_rad: f64,
    pub target_rad: f64,
    pub angular_rate_rps: f64,
    pub maximum_rate_rps: f64,
    pub maximum_acceleration_rps2: f64,
}

impl Default for PanelController {
    fn default() -> Self {
        Self {
            angle_rad: 0.0,
            target_rad: 0.0,
            angular_rate_rps: 0.0,
            maximum_rate_rps: 0.05,
            maximum_acceleration_rps2: 0.01,
        }
    }
}

impl PanelController {
    pub fn target(&mut self, angle_rad: f64) -> Result<(), ControlError> {
        self.target_rad = finite_in_range(angle_rad, -1.4, 1.4)?;
        Ok(())
    }

    pub fn tick(&mut self, dt_s: f64, actuator_available: bool) -> Result<f64, ControlError> {
        finite_in_range(dt_s, 0.001, 1.0)?;
        finite_in_range(self.maximum_rate_rps, 0.001, 0.2)?;
        finite_in_range(self.maximum_acceleration_rps2, 0.001, 0.1)?;
        if !actuator_available {
            self.angular_rate_rps = 0.0;
            return Err(ControlError::InterlockOpen);
        }
        let error = self.target_rad - self.angle_rad;
        let stopping_rate = (2.0 * self.maximum_acceleration_rps2 * error.abs()).sqrt();
        let requested = error.signum() * stopping_rate.min(self.maximum_rate_rps);
        let acceleration_step = self.maximum_acceleration_rps2 * dt_s;
        self.angular_rate_rps +=
            (requested - self.angular_rate_rps).clamp(-acceleration_step, acceleration_step);
        let movement = self.angular_rate_rps * dt_s;
        if movement.abs() >= error.abs() {
            self.angle_rad = self.target_rad;
            self.angular_rate_rps = 0.0;
        } else {
            self.angle_rad += movement;
        }
        Ok(self.angle_rad)
    }
}

pub fn stopping_distance_rad(
    rate_rps: f64,
    acceleration_rps2: f64,
) -> Result<f64, crate::ControlError> {
    crate::finite_in_range(rate_rps, -0.2, 0.2)?;
    crate::finite_in_range(acceleration_rps2, 0.001, 0.1)?;
    Ok(rate_rps * rate_rps / (2.0 * acceleration_rps2))
}
