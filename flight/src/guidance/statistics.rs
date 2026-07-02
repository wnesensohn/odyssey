use crate::ControlError;

pub fn median(values: &[f64]) -> Result<f64, ControlError> {
    if values.is_empty() || values.iter().any(|value| !value.is_finite()) {
        return Err(ControlError::InvalidSample);
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let middle = sorted.len() / 2;
    Ok(if sorted.len() % 2 == 0 {
        (sorted[middle - 1] + sorted[middle]) / 2.0
    } else {
        sorted[middle]
    })
}
