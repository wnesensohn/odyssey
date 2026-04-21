use odyssey_flight::propulsion::tank::*;

#[test]
fn nominal_behavior() {
    assert_eq!(usable_propellant_kg(100.0, 20.0, 290.0).unwrap(), 80.0);
}
