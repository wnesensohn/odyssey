#[derive(Debug, Clone, Copy)]
pub struct ReserveSchedule {
    pub capacity_wh: f64,
    pub fraction: f64,
    pub reserve_fraction: f64,
}
