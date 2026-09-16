use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone)]
pub struct CoolantLoop {
    pub inlet_k: f64,
    pub outlet_k: f64,
    pub flow_kgps: f64,
    pub heat_capacity_jkgk: f64,
}

impl CoolantLoop {
    pub fn heat_removed_w(&self) -> Result<f64, ControlError> {
        finite_in_range(self.inlet_k, 200.0, 500.0)?;
        finite_in_range(self.outlet_k, 200.0, 500.0)?;
        finite_in_range(self.flow_kgps, 0.0, 20.0)?;
        finite_in_range(self.heat_capacity_jkgk, 100.0, 10_000.0)?;
        Ok(self.flow_kgps * self.heat_capacity_jkgk * (self.outlet_k - self.inlet_k))
    }
    pub fn required_flow(&self, load_w: f64, maximum_rise_k: f64) -> Result<f64, ControlError> {
        finite_in_range(load_w, 0.0, 100_000.0)?;
        finite_in_range(maximum_rise_k, 0.1, 100.0)?;
        finite_in_range(self.heat_capacity_jkgk, 100.0, 10_000.0)?;
        Ok(load_w / (self.heat_capacity_jkgk * maximum_rise_k))
    }
}

pub fn temperature_settled(
    samples_k: &[f64],
    tolerance_k: f64,
) -> Result<bool, crate::ControlError> {
    crate::finite_in_range(tolerance_k, 0.01, 10.0)?;
    if samples_k.len() < 5 {
        return Err(crate::ControlError::InvalidSample);
    }
    let tail = &samples_k[samples_k.len() - 5..];
    for sample in tail {
        crate::finite_in_range(*sample, 150.0, 500.0)?;
    }
    let low = tail.iter().copied().fold(f64::INFINITY, f64::min);
    let high = tail.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    Ok(high - low <= tolerance_k)
}

pub fn cooldown_stalled(
    samples_k: &[f64],
    minimum_drop_k: f64,
) -> Result<bool, crate::ControlError> {
    crate::finite_in_range(minimum_drop_k, 0.0, 100.0)?;
    if samples_k.len() < 3 || samples_k.iter().any(|v| !v.is_finite()) {
        return Err(crate::ControlError::InvalidSample);
    }
    Ok(samples_k[0] - samples_k[samples_k.len() - 1] < minimum_drop_k)
}
