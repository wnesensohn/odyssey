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
