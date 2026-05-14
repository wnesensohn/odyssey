use odyssey_flight::guidance::estimator::*;

#[test]
fn nominal_behavior() {
    assert_eq!(innovation_score(6.0, 4.0).unwrap(), 3.0);
}
