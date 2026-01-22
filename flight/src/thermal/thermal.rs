use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone)]
pub struct Heater {
    pub enabled: bool,
    pub low_k: f64,
    pub high_k: f64,
    pub maximum_power_w: f64,
}

impl Heater {
    pub fn new(low_k: f64, high_k: f64, maximum_power_w: f64) -> Result<Self, ControlError> {
        finite_in_range(low_k, 150.0, 400.0)?;
        finite_in_range(high_k, low_k + 1.0, 450.0)?;
        finite_in_range(maximum_power_w, 0.0, 5000.0)?;
        Ok(Self {
            enabled: false,
            low_k,
            high_k,
            maximum_power_w,
        })
    }

    pub fn update(&mut self, temperature_k: f64, bus_available: bool) -> Result<f64, ControlError> {
        finite_in_range(temperature_k, 0.0, 1000.0)?;
        if !bus_available || temperature_k >= self.high_k {
            self.enabled = false;
        } else if temperature_k <= self.low_k {
            self.enabled = true;
        }
        Ok(if self.enabled {
            self.maximum_power_w
        } else {
            0.0
        })
    }
}

pub fn radiative_loss(
    area_m2: f64,
    emissivity: f64,
    surface_k: f64,
    environment_k: f64,
) -> Result<f64, ControlError> {
    finite_in_range(area_m2, 0.0, 1000.0)?;
    finite_in_range(emissivity, 0.0, 1.0)?;
    finite_in_range(surface_k, 0.0, 2000.0)?;
    finite_in_range(environment_k, 0.0, 2000.0)?;
    Ok(5.670_374_419e-8 * area_m2 * emissivity * (surface_k.powi(4) - environment_k.powi(4)))
}

pub fn heater_energy_wh(
    power_w: f64,
    interval_s: f64,
    budget_wh: f64,
) -> Result<f64, crate::ControlError> {
    crate::finite_in_range(power_w, 0.0, 5000.0)?;
    crate::finite_in_range(interval_s, 0.0, 3600.0)?;
    crate::finite_in_range(budget_wh, 0.0, 5000.0)?;
    let energy = power_w * interval_s / 3600.0;
    if energy > budget_wh {
        return Err(crate::ControlError::InterlockOpen);
    }
    Ok(energy)
}
