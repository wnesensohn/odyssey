use odyssey_flight::power::power::*;

#[test]
fn nominal_behavior() {
    assert!(restore_order(&[], 100.0, 20.0).unwrap().is_empty());
}
