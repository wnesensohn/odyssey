use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sample {
    pub value: f64,
    pub timestamp_ms: u64,
    pub quality: Quality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    Good,
    Degraded,
    Invalid,
}

#[derive(Debug, Clone, Copy)]
pub struct SensorLimits {
    pub minimum: f64,
    pub maximum: f64,
    pub maximum_age_ms: u64,
}

impl SensorLimits {
    pub fn validate(&self, sample: Sample, now_ms: u64) -> Result<f64, ControlError> {
        if sample.quality == Quality::Invalid || sample.timestamp_ms > now_ms {
            return Err(ControlError::InvalidSample);
        }
        if now_ms - sample.timestamp_ms > self.maximum_age_ms {
            return Err(ControlError::StaleSample);
        }
        finite_in_range(sample.value, self.minimum, self.maximum)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Calibration {
    pub gain: f64,
    pub offset: f64,
}

impl Calibration {
    pub fn apply(&self, raw: f64) -> Result<f64, ControlError> {
        if !raw.is_finite() || !self.gain.is_finite() || !self.offset.is_finite() {
            return Err(ControlError::InvalidSample);
        }
        let calibrated = raw * self.gain + self.offset;
        if !calibrated.is_finite() {
            return Err(ControlError::InvalidSample);
        }
        Ok(calibrated)
    }
}
