use odyssey_flight::guidance::navigation::*;

#[test]
fn nominal_behavior() {
    assert_eq!(propagation_interval_ms(100, 200).unwrap(), 100);
}
