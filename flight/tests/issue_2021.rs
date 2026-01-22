use odyssey_flight::thermal::thermal::*;

#[test]
fn nominal_behavior() {
    assert_eq!(heater_energy_wh(100.0, 36.0, 2.0).unwrap(), 1.0);
}

#[test]
fn boundary_behavior() {
    assert!(heater_energy_wh(100.0, 360.0, 2.0).is_err());
}

#[test]
fn invalid_behavior() {
    assert!(heater_energy_wh(f64::INFINITY, 1.0, 1.0).is_err());
}
