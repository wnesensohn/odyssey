use odyssey_flight::avionics::watchdog::*;

#[test]
fn nominal_behavior() {
    assert!(recovery_quorum(&[true, true, false], 2).unwrap());
}

#[test]
fn boundary_behavior() {
    assert!(!recovery_quorum(&[true, false, false], 2).unwrap());
}

#[test]
fn invalid_behavior() {
    assert!(recovery_quorum(&[true], 0).is_err());
}
