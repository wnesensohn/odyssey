use odyssey_flight::propulsion::pump::*;

#[test]
fn nominal_behavior() {
    assert_eq!(cavitation_margin_kpa(100.0, 25.0).unwrap(), 75.0);
}

#[test]
fn boundary_behavior() {
    assert_eq!(cavitation_margin_kpa(50.0, 25.0).unwrap(), 25.0);
}

#[test]
fn invalid_behavior() {
    assert!(cavitation_margin_kpa(30.0, 25.0).is_err());
}
