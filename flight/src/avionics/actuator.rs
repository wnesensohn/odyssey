use crate::{finite_in_range, ControlError};

pub trait ActuatorBus {
    fn write_duty(&mut self, channel: u8, duty: f64) -> Result<(), ControlError>;
    fn read_current(&self, channel: u8) -> Result<f64, ControlError>;
}

#[derive(Debug, Clone)]
pub struct SimulatedBus {
    channels: [f64; 16],
    fail_mask: u16,
}

impl Default for SimulatedBus {
    fn default() -> Self {
        Self {
            channels: [0.0; 16],
            fail_mask: 0,
        }
    }
}

impl SimulatedBus {
    pub fn inject_failure(&mut self, channel: u8) -> Result<(), ControlError> {
        if channel >= 16 {
            return Err(ControlError::OutOfRange);
        }
        self.fail_mask |= 1 << channel;
        Ok(())
    }
    pub fn clear_failures(&mut self) {
        self.fail_mask = 0;
    }
    pub fn duty(&self, channel: u8) -> Option<f64> {
        self.channels.get(channel as usize).copied()
    }
}

impl ActuatorBus for SimulatedBus {
    fn write_duty(&mut self, channel: u8, duty: f64) -> Result<(), ControlError> {
        finite_in_range(duty, 0.0, 1.0)?;
        if channel >= 16 {
            return Err(ControlError::OutOfRange);
        }
        if self.fail_mask & (1 << channel) != 0 {
            return Err(ControlError::InterlockOpen);
        }
        self.channels[channel as usize] = duty;
        Ok(())
    }
    fn read_current(&self, channel: u8) -> Result<f64, ControlError> {
        self.duty(channel)
            .map(|duty| duty * 12.0)
            .ok_or(ControlError::OutOfRange)
    }
}
