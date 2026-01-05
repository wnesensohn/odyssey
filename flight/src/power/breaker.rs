use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakerState {
    Open,
    Closed,
    Tripped,
}

#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    pub state: BreakerState,
    pub rated_current_a: f64,
    pub instantaneous_limit_a: f64,
    pub thermal_limit_a2s: f64,
    pub thermal_accumulator_a2s: f64,
}

impl CircuitBreaker {
    pub fn new(rated_current_a: f64) -> Result<Self, ControlError> {
        finite_in_range(rated_current_a, 0.1, 100.0)?;
        Ok(Self {
            state: BreakerState::Open,
            rated_current_a,
            instantaneous_limit_a: rated_current_a * 3.0,
            thermal_limit_a2s: rated_current_a.powi(2) * 5.0,
            thermal_accumulator_a2s: 0.0,
        })
    }
    pub fn close(&mut self) -> Result<(), ControlError> {
        if self.state == BreakerState::Tripped {
            return Err(ControlError::InterlockOpen);
        }
        self.state = BreakerState::Closed;
        Ok(())
    }
    pub fn open(&mut self) {
        self.state = BreakerState::Open;
    }
    pub fn update(&mut self, current_a: f64, dt_s: f64) -> Result<BreakerState, ControlError> {
        finite_in_range(current_a, 0.0, 1000.0)?;
        finite_in_range(dt_s, 0.001, 1.0)?;
        if self.state != BreakerState::Closed {
            return Ok(self.state);
        }
        let excess = (current_a.powi(2) - self.rated_current_a.powi(2)).max(0.0);
        let cooling = self.rated_current_a.powi(2) * dt_s * 0.1;
        self.thermal_accumulator_a2s =
            (self.thermal_accumulator_a2s + excess * dt_s - cooling).max(0.0);
        if current_a > self.instantaneous_limit_a
            || self.thermal_accumulator_a2s > self.thermal_limit_a2s
        {
            self.state = BreakerState::Tripped;
        }
        Ok(self.state)
    }
    pub fn reset(&mut self, current_a: f64) -> Result<(), ControlError> {
        finite_in_range(current_a, 0.0, 0.01)?;
        if self.state != BreakerState::Tripped {
            return Err(ControlError::InvalidTransition);
        }
        self.state = BreakerState::Open;
        self.thermal_accumulator_a2s = 0.0;
        Ok(())
    }
}
