use odyssey_flight::propulsion::feed::*;

#[test]
fn nominal_behavior() {
    assert!(purge_complete(20.0, 300.0, 1000).unwrap());
}
