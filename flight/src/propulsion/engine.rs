use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineState {
    Safe,
    Priming,
    Igniting,
    Running,
    Cooling,
    Fault,
}

#[derive(Debug, Clone, Copy)]
pub struct EngineInput {
    pub feed_pressure_kpa: f64,
    pub chamber_temperature_k: f64,
    pub valves_ready: bool,
    pub ignition_confirmed: bool,
    pub stop_requested: bool,
}

#[derive(Debug, Clone)]
pub struct EngineController {
    state: EngineState,
    throttle: f64,
    commanded_throttle: f64,
    elapsed_ms: u64,
}

impl Default for EngineController {
    fn default() -> Self {
        Self {
            state: EngineState::Safe,
            throttle: 0.0,
            commanded_throttle: 0.0,
            elapsed_ms: 0,
        }
    }
}

impl EngineController {
    pub fn state(&self) -> EngineState {
        self.state
    }
    pub fn throttle(&self) -> f64 {
        self.throttle
    }

    pub fn arm(&mut self) -> Result<(), ControlError> {
        if self.state != EngineState::Safe {
            return Err(ControlError::InvalidTransition);
        }
        self.state = EngineState::Priming;
        self.elapsed_ms = 0;
        Ok(())
    }

    pub fn set_throttle(&mut self, target: f64) -> Result<(), ControlError> {
        self.commanded_throttle = finite_in_range(target, 0.0, 1.0)?;
        Ok(())
    }

    pub fn tick(&mut self, input: EngineInput, dt_ms: u64) -> Result<EngineState, ControlError> {
        finite_in_range(input.feed_pressure_kpa, 0.0, 1500.0)?;
        finite_in_range(input.chamber_temperature_k, 0.0, 1800.0)?;
        if dt_ms == 0 || dt_ms > 1000 {
            return Err(ControlError::OutOfRange);
        }
        self.elapsed_ms = self.elapsed_ms.saturating_add(dt_ms);
        if input.chamber_temperature_k > 1250.0 {
            self.state = EngineState::Fault;
            self.throttle = 0.0;
            return Err(ControlError::InterlockOpen);
        }
        if input.stop_requested
            && matches!(self.state, EngineState::Running | EngineState::Igniting)
        {
            self.state = EngineState::Cooling;
            self.elapsed_ms = 0;
        }
        match self.state {
            EngineState::Priming if input.valves_ready && input.feed_pressure_kpa >= 250.0 => {
                self.state = EngineState::Igniting;
                self.elapsed_ms = 0;
            }
            EngineState::Igniting if input.ignition_confirmed => {
                self.state = EngineState::Running;
                self.elapsed_ms = 0;
            }
            EngineState::Igniting if self.elapsed_ms > 3000 => {
                self.state = EngineState::Fault;
                self.throttle = 0.0;
                return Err(ControlError::InterlockOpen);
            }
            EngineState::Running => {
                if input.feed_pressure_kpa < 180.0 {
                    self.state = EngineState::Fault;
                    self.throttle = 0.0;
                    return Err(ControlError::InterlockOpen);
                }
                let step = dt_ms as f64 / 1000.0 * 0.2;
                self.throttle += (self.commanded_throttle - self.throttle).clamp(-step, step);
            }
            EngineState::Cooling => {
                self.throttle = 0.0;
                if input.chamber_temperature_k < 450.0 {
                    self.state = EngineState::Safe;
                }
            }
            _ => {}
        }
        Ok(self.state)
    }

    pub fn reset_fault(&mut self, temperature_k: f64) -> Result<(), ControlError> {
        finite_in_range(temperature_k, 0.0, 450.0)?;
        if self.state != EngineState::Fault {
            return Err(ControlError::InvalidTransition);
        }
        *self = Self::default();
        Ok(())
    }
}
