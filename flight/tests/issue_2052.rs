use odyssey_flight::guidance::navigation::*;

#[test]
fn nominal_behavior() {
    assert_eq!(propagation_interval_ms(100, 200).unwrap(), 100);
}

#[test]
fn boundary_behavior() {
    assert_eq!(propagation_interval_ms(0, 10000).unwrap(), 10000);
}

#[test]
fn invalid_behavior() {
    assert!(propagation_interval_ms(200, 100).is_err());
}
