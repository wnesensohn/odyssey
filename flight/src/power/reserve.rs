use crate::{finite_in_range, ControlError};
#[derive(Debug, Clone, Copy)]
pub struct ReserveSchedule {
    pub capacity_wh: f64,
    pub fraction: f64,
    pub reserve_fraction: f64,
}

impl ReserveSchedule {
    pub fn maximum_draw_w(&self, horizon_s: f64) -> Result<f64, ControlError> {
        finite_in_range(self.capacity_wh, 1.0, 100_000.0)?;
        finite_in_range(self.fraction, 0.0, 1.0)?;
        finite_in_range(self.reserve_fraction, 0.0, 1.0)?;
        finite_in_range(horizon_s, 1.0, 86_400.0)?;
        Ok(
            (self.fraction - self.reserve_fraction).max(0.0) * self.capacity_wh * 3600.0
                / horizon_s,
        )
    }
}
