use odyssey_flight::thermal::thermal::*;

#[test]
fn nominal_behavior() {
    assert_eq!(
        voted_temperature(&[290.0, 291.0, 400.0], 2.0).unwrap(),
        290.5
    );
}

#[test]
fn boundary_behavior() {
    assert!(voted_temperature(&[200.0, 300.0, 400.0], 1.0).is_err());
}

#[test]
fn invalid_behavior() {
    assert!(voted_temperature(&[f64::NAN, 290.0, 291.0], 2.0).is_err());
}
