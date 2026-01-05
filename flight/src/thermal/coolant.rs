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
