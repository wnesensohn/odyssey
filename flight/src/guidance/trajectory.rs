use super::navigation::EARTH_MU;
use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone, Copy)]
pub struct Transfer {
    pub first_burn_mps: f64,
    pub second_burn_mps: f64,
    pub coast_s: f64,
}

pub fn circular_transfer(
    initial_radius_m: f64,
    target_radius_m: f64,
) -> Result<Transfer, ControlError> {
    finite_in_range(initial_radius_m, 6_400_000.0, 1e9)?;
    finite_in_range(target_radius_m, 6_400_000.0, 1e9)?;
    let major_axis = (initial_radius_m + target_radius_m) / 2.0;
    let initial_speed = (EARTH_MU / initial_radius_m).sqrt();
    let target_speed = (EARTH_MU / target_radius_m).sqrt();
    let transfer_start = (EARTH_MU * (2.0 / initial_radius_m - 1.0 / major_axis)).sqrt();
    let transfer_end = (EARTH_MU * (2.0 / target_radius_m - 1.0 / major_axis)).sqrt();
    Ok(Transfer {
        first_burn_mps: transfer_start - initial_speed,
        second_burn_mps: target_speed - transfer_end,
        coast_s: std::f64::consts::PI * (major_axis.powi(3) / EARTH_MU).sqrt(),
    })
}

pub fn propellant_required(
    dry_mass_kg: f64,
    delta_v_mps: f64,
    specific_impulse_s: f64,
) -> Result<f64, ControlError> {
    finite_in_range(dry_mass_kg, 1.0, 1e7)?;
    finite_in_range(delta_v_mps, 0.0, 20_000.0)?;
    finite_in_range(specific_impulse_s, 10.0, 5000.0)?;
    Ok(dry_mass_kg * ((delta_v_mps / (specific_impulse_s * 9.80665)).exp() - 1.0))
}

pub fn coast_checkpoints(coast_s: f64, segments: usize) -> Result<Vec<f64>, crate::ControlError> {
    crate::finite_in_range(coast_s, 0.1, 1e7)?;
    if segments == 0 || segments > 1000 {
        return Err(crate::ControlError::OutOfRange);
    }
    Ok((0..=segments)
        .map(|index| coast_s * index as f64 / segments as f64)
        .collect())
}
