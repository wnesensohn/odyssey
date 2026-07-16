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
