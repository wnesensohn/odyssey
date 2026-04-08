use odyssey_flight::guidance::shaping::*;

#[test]
fn nominal_behavior() {
    assert!((stopping_distance_rad(0.05, 0.01).unwrap() - 0.125).abs() < 1e-9);
}
