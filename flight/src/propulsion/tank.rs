use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone)]
pub struct Tank {
    pub capacity_kg: f64,
    pub remaining_kg: f64,
    pub temperature_k: f64,
    pub ullage_m3: f64,
    pub pressurant_mol: f64,
}

impl Tank {
    pub fn pressure_kpa(&self) -> Result<f64, ControlError> {
        finite_in_range(self.temperature_k, 150.0, 500.0)?;
        finite_in_range(self.ullage_m3, 0.001, 100.0)?;
        finite_in_range(self.pressurant_mol, 0.0, 100_000.0)?;
        Ok(self.pressurant_mol * 8.314_462_618 * self.temperature_k / self.ullage_m3 / 1000.0)
    }
    pub fn consume(&mut self, mass_flow_kgps: f64, dt_s: f64) -> Result<f64, ControlError> {
        finite_in_range(mass_flow_kgps, 0.0, 100.0)?;
        finite_in_range(dt_s, 0.0, 10.0)?;
        finite_in_range(self.remaining_kg, 0.0, self.capacity_kg)?;
        finite_in_range(self.capacity_kg, 0.01, 1e6)?;
        let required = mass_flow_kgps * dt_s;
        if required > self.remaining_kg {
            return Err(ControlError::InterlockOpen);
        }
        self.remaining_kg -= required;
        Ok(self.remaining_kg)
    }
    pub fn reserve_fraction(&self) -> Result<f64, ControlError> {
        finite_in_range(self.capacity_kg, 0.01, 1e6)?;
        finite_in_range(self.remaining_kg, 0.0, self.capacity_kg)?;
        Ok(self.remaining_kg / self.capacity_kg)
    }
}

pub fn usable_propellant_kg(
    remaining_kg: f64,
    reserve_kg: f64,
    temperature_k: f64,
) -> Result<f64, crate::ControlError> {
    crate::finite_in_range(remaining_kg, 0.0, 1e6)?;
    crate::finite_in_range(reserve_kg, 0.0, remaining_kg)?;
    crate::finite_in_range(temperature_k, 250.0, 330.0)?;
    Ok(remaining_kg - reserve_kg)
}
