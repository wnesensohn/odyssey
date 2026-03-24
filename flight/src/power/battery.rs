use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone)]
pub struct Battery {
    capacity_wh: f64,
    stored_wh: f64,
    pub temperature_k: f64,
}

impl Battery {
    pub fn new(capacity_wh: f64, initial_fraction: f64) -> Result<Self, ControlError> {
        finite_in_range(capacity_wh, 1.0, 100_000.0)?;
        finite_in_range(initial_fraction, 0.0, 1.0)?;
        Ok(Self {
            capacity_wh,
            stored_wh: capacity_wh * initial_fraction,
            temperature_k: 293.15,
        })
    }

    pub fn fraction(&self) -> f64 {
        self.stored_wh / self.capacity_wh
    }
    pub fn reserve_available(&self) -> bool {
        self.fraction() >= 0.20
    }

    pub fn integrate(&mut self, bus_power_w: f64, seconds: f64) -> Result<f64, ControlError> {
        finite_in_range(bus_power_w, -20_000.0, 20_000.0)?;
        finite_in_range(seconds, 0.0, 60.0)?;
        finite_in_range(self.temperature_k, 250.0, 330.0)?;
        let efficiency = if bus_power_w > 0.0 { 0.94 } else { 1.0 / 0.96 };
        self.stored_wh = (self.stored_wh + bus_power_w * seconds / 3600.0 * efficiency)
            .clamp(0.0, self.capacity_wh);
        Ok(self.fraction())
    }
}

pub fn reserve_band(fraction: f64) -> Result<&'static str, crate::ControlError> {
    crate::finite_in_range(fraction, 0.0, 1.0)?;
    Ok(if fraction < 0.10 {
        "critical"
    } else if fraction < 0.20 {
        "reserve"
    } else {
        "nominal"
    })
}

pub fn charge_acceptance_fraction(temperature_k: f64) -> Result<f64, crate::ControlError> {
    crate::finite_in_range(temperature_k, 200.0, 400.0)?;
    if !(273.15..=323.15).contains(&temperature_k) {
        return Ok(0.0);
    }
    Ok(((temperature_k - 273.15) / 10.0)
        .min((323.15 - temperature_k) / 10.0)
        .clamp(0.0, 1.0))
}
