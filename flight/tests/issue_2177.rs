use odyssey_flight::thermal::coolant::*;

#[test]
fn nominal_behavior() {
    assert!(!cooldown_stalled(&[320.0, 310.0, 300.0], 5.0).unwrap());
}

#[test]
fn boundary_behavior() {
    assert!(cooldown_stalled(&[320.0, 320.0, 320.0], 5.0).unwrap());
}

#[test]
fn invalid_behavior() {
    assert!(cooldown_stalled(&[f64::NAN, 300.0, 290.0], 5.0).is_err());
}
