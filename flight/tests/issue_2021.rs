use odyssey_flight::thermal::thermal::*;

#[test]
fn nominal_behavior() {
    assert_eq!(heater_energy_wh(100.0, 36.0, 2.0).unwrap(), 1.0);
}
