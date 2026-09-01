use odyssey_flight::power::reserve::ReserveSchedule;
#[test]
fn reserve_is_preserved_over_a_one_hour_coast() {
    let schedule = ReserveSchedule {
        capacity_wh: 1000.0,
        fraction: 0.8,
        reserve_fraction: 0.2,
    };
    assert!((schedule.maximum_draw_w(3600.0).unwrap() - 600.0).abs() < 1e-9);
}
