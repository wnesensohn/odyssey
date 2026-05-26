use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone)]
pub struct ScalarEstimator {
    pub estimate: f64,
    pub variance: f64,
    pub process_variance_per_second: f64,
    pub innovation_limit_sigma: f64,
    pub rejected: u64,
}

impl ScalarEstimator {
    pub fn new(estimate: f64, variance: f64) -> Result<Self, ControlError> {
        if !estimate.is_finite() {
            return Err(ControlError::InvalidSample);
        }
        finite_in_range(variance, 1e-12, 1e12)?;
        Ok(Self {
            estimate,
            variance,
            process_variance_per_second: 0.01,
            innovation_limit_sigma: 5.0,
            rejected: 0,
        })
    }
    pub fn predict(&mut self, rate: f64, dt_s: f64) -> Result<f64, ControlError> {
        if !rate.is_finite() {
            return Err(ControlError::InvalidSample);
        }
        finite_in_range(dt_s, 0.0, 60.0)?;
        finite_in_range(self.process_variance_per_second, 0.0, 1000.0)?;
        self.estimate += rate * dt_s;
        self.variance += self.process_variance_per_second * dt_s;
        Ok(self.estimate)
    }
    pub fn observe(
        &mut self,
        sample: f64,
        measurement_variance: f64,
    ) -> Result<bool, ControlError> {
        if !sample.is_finite() {
            return Err(ControlError::InvalidSample);
        }
        finite_in_range(measurement_variance, 1e-12, 1e12)?;
        finite_in_range(self.innovation_limit_sigma, 1.0, 20.0)?;
        let residual = sample - self.estimate;
        let innovation_variance = self.variance + measurement_variance;
        if residual.abs() > self.innovation_limit_sigma * innovation_variance.sqrt() {
            self.rejected = self.rejected.saturating_add(1);
            return Ok(false);
        }
        let gain = self.variance / innovation_variance;
        self.estimate += gain * residual;
        self.variance = (1.0 - gain) * self.variance;
        Ok(true)
    }
}

pub fn innovation_score(residual: f64, variance: f64) -> Result<f64, crate::ControlError> {
    if !residual.is_finite() {
        return Err(crate::ControlError::InvalidSample);
    }
    crate::finite_in_range(variance, 1e-12, 1e12)?;
    Ok(residual.abs() / variance.sqrt())
}
