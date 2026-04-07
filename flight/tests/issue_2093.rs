use odyssey_flight::avionics::watchdog::*;

#[test]
fn nominal_behavior() {
    assert!(recovery_quorum(&[true, true, false], 2).unwrap());
}
