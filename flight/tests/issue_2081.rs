use odyssey_flight::power::battery::*;

#[test]
fn nominal_behavior() {
    assert_eq!(charge_acceptance_fraction(293.15).unwrap(), 1.0);
}

#[test]
fn boundary_behavior() {
    assert_eq!(charge_acceptance_fraction(260.0).unwrap(), 0.0);
}

#[test]
fn invalid_behavior() {
    assert!(charge_acceptance_fraction(f64::NAN).is_err());
}
