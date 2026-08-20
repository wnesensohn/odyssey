use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone)]
pub struct PressureWindow {
    samples: Vec<f64>,
    capacity: usize,
}

impl PressureWindow {
    pub fn new(capacity: usize) -> Result<Self, ControlError> {
        if capacity == 0 || capacity > 256 {
            return Err(ControlError::OutOfRange);
        }
        Ok(Self {
            samples: Vec::with_capacity(capacity),
            capacity,
        })
    }

    pub fn push(&mut self, pressure_kpa: f64) -> Result<f64, ControlError> {
        finite_in_range(pressure_kpa, 0.0, 1500.0)?;
        if self.samples.len() == self.capacity {
            self.samples.remove(0);
        }
        self.samples.push(pressure_kpa);
        Ok(self.samples.iter().sum::<f64>() / self.samples.len() as f64)
    }

    pub fn spread(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let low = self.samples.iter().copied().fold(f64::INFINITY, f64::min);
        let high = self
            .samples
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        high - low
    }
}

pub fn pressure_permits_ignition(value_kpa: f64, minimum_kpa: f64) -> bool {
    value_kpa.is_finite() && minimum_kpa.is_finite() && value_kpa >= minimum_kpa
}

pub fn fresh_pressure_permits_ignition(
    sample: crate::sensor::Sample,
    now_ms: u64,
    minimum_kpa: f64,
    maximum_age_ms: u64,
) -> bool {
    sample.quality == crate::sensor::Quality::Good
        && sample.timestamp_ms <= now_ms
        && now_ms - sample.timestamp_ms <= maximum_age_ms
        && pressure_permits_ignition(sample.value, minimum_kpa + 15.0)
}
