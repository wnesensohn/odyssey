use odyssey_flight::propulsion::tank::*;

#[test]
fn nominal_behavior() {
    assert_eq!(usable_propellant_kg(100.0, 20.0, 290.0).unwrap(), 80.0);
}

#[test]
fn boundary_behavior() {
    assert_eq!(usable_propellant_kg(100.0, 100.0, 290.0).unwrap(), 0.0);
}

#[test]
fn invalid_behavior() {
    assert!(usable_propellant_kg(10.0, 20.0, 290.0).is_err());
}
