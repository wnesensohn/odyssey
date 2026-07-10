use crate::ControlError;

#[derive(Debug, Clone)]
pub struct LowPass {
    value: Option<f64>,
    time_constant_s: f64,
}

impl LowPass {
    pub fn update(&mut self, sample: f64, dt_s: f64) -> Result<f64, ControlError> {
        if !sample.is_finite() || !dt_s.is_finite() || dt_s <= 0.0 {
            return Err(ControlError::InvalidSample);
        }
        let alpha = dt_s / (self.time_constant_s + dt_s);
        let filtered = self
            .value
            .map_or(sample, |previous| previous + alpha * (sample - previous));
        self.value = Some(filtered);
        Ok(filtered)
    }

    pub fn new(time_constant_s: f64) -> Result<Self, ControlError> {
        if !time_constant_s.is_finite() || time_constant_s <= 0.0 {
            return Err(ControlError::OutOfRange);
        }
        Ok(Self {
            value: None,
            time_constant_s,
        })
    }

    pub fn reset(&mut self) {
        self.value = None;
    }
}

pub use super::statistics::median;
