use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PumpState {
    Stopped,
    Starting,
    Regulating,
    Tripped,
}

#[derive(Debug, Clone)]
pub struct PumpController {
    pub state: PumpState,
    pub target_kpa: f64,
    pub duty: f64,
    integral: f64,
}

impl PumpController {
    pub fn new(target_kpa: f64) -> Result<Self, ControlError> {
        finite_in_range(target_kpa, 50.0, 800.0)?;
        Ok(Self {
            state: PumpState::Stopped,
            target_kpa,
            duty: 0.0,
            integral: 0.0,
        })
    }

    pub fn start(&mut self) -> Result<(), ControlError> {
        if self.state != PumpState::Stopped {
            return Err(ControlError::InvalidTransition);
        }
        self.state = PumpState::Starting;
        self.integral = 0.0;
        Ok(())
    }

    pub fn stop(&mut self) {
        self.state = PumpState::Stopped;
        self.duty = 0.0;
        self.integral = 0.0;
    }

    pub fn regulate(
        &mut self,
        pressure_kpa: f64,
        current_a: f64,
        dt_s: f64,
    ) -> Result<f64, ControlError> {
        finite_in_range(pressure_kpa, 0.0, 1000.0)?;
        finite_in_range(current_a, 0.0, 30.0)?;
        finite_in_range(dt_s, 0.001, 1.0)?;
        if current_a > 18.0 || pressure_kpa > 850.0 {
            self.state = PumpState::Tripped;
            self.duty = 0.0;
            return Err(ControlError::InterlockOpen);
        }
        if self.state == PumpState::Stopped {
            return Ok(0.0);
        }
        if self.state == PumpState::Tripped {
            return Err(ControlError::InvalidTransition);
        }
        let error = self.target_kpa - pressure_kpa;
        self.integral = (self.integral + error * dt_s).clamp(-100.0, 100.0);
        let requested = (error * 0.003 + self.integral * 0.0004).clamp(0.0, 1.0);
        let step = dt_s * 0.5;
        self.duty += (requested - self.duty).clamp(-step, step);
        if pressure_kpa >= self.target_kpa * 0.85 {
            self.state = PumpState::Regulating;
        }
        Ok(self.duty)
    }
}
