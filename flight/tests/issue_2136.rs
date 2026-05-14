use odyssey_flight::guidance::estimator::*;

#[test]
fn nominal_behavior() {
    assert_eq!(innovation_score(6.0, 4.0).unwrap(), 3.0);
}

#[test]
fn boundary_behavior() {
    assert_eq!(innovation_score(0.0, 1.0).unwrap(), 0.0);
}

#[test]
fn invalid_behavior() {
    assert!(innovation_score(1.0, 0.0).is_err());
}
