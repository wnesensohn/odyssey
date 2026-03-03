use odyssey_flight::thermal::thermal::*;

#[test]
fn nominal_behavior() {
    assert_eq!(
        voted_temperature(&[290.0, 291.0, 400.0], 2.0).unwrap(),
        290.5
    );
}
