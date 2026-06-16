use odyssey_flight::thermal::coolant::*;

#[test]
fn nominal_behavior() {
    assert!(!cooldown_stalled(&[320.0, 310.0, 300.0], 5.0).unwrap());
}
