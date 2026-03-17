use odyssey_flight::propulsion::pump::*;

#[test]
fn nominal_behavior() {
    assert_eq!(cavitation_margin_kpa(100.0, 25.0).unwrap(), 75.0);
}
