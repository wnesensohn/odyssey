use odyssey_flight::power::power::*;

#[test]
fn nominal_behavior() {
    assert_eq!(illumination_state(0.0, false).unwrap(), "illuminated");
}

#[test]
fn boundary_behavior() {
    assert_eq!(illumination_state(0.0, true).unwrap(), "eclipse");
}

#[test]
fn invalid_behavior() {
    assert!(illumination_state(f64::NAN, false).is_err());
}
