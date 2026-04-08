use odyssey_flight::guidance::shaping::*;

#[test]
fn nominal_behavior() {
    assert!((stopping_distance_rad(0.05, 0.01).unwrap() - 0.125).abs() < 1e-9);
}

#[test]
fn boundary_behavior() {
    assert_eq!(stopping_distance_rad(0.0, 0.01).unwrap(), 0.0);
}

#[test]
fn invalid_behavior() {
    assert!(stopping_distance_rad(0.05, 0.0).is_err());
}
