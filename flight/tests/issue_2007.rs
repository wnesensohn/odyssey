use odyssey_flight::power::battery::*;

#[test]
fn nominal_behavior() {
    assert_eq!(reserve_band(0.8).unwrap(), "nominal");
}

#[test]
fn boundary_behavior() {
    assert_eq!(reserve_band(0.15).unwrap(), "reserve");
}

#[test]
fn invalid_behavior() {
    assert!(reserve_band(f64::NAN).is_err());
}
