use odyssey_flight::propulsion::feed::*;

#[test]
fn nominal_behavior() {
    assert!(purge_complete(20.0, 300.0, 1000).unwrap());
}

#[test]
fn boundary_behavior() {
    assert!(!purge_complete(100.0, 300.0, 1000).unwrap());
}

#[test]
fn invalid_behavior() {
    assert!(purge_complete(f64::NAN, 300.0, 1000).is_err());
}
