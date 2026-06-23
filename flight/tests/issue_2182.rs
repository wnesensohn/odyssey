use odyssey_flight::guidance::trajectory::*;

#[test]
fn nominal_behavior() {
    assert_eq!(coast_checkpoints(100.0, 2).unwrap(), vec![0.0, 50.0, 100.0]);
}

#[test]
fn boundary_behavior() {
    assert_eq!(coast_checkpoints(1.0, 1).unwrap(), vec![0.0, 1.0]);
}

#[test]
fn invalid_behavior() {
    assert!(coast_checkpoints(100.0, 0).is_err());
}
