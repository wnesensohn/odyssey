use odyssey_flight::thermal::coolant::temperature_settled;
#[test]
fn five_consistent_samples_are_settled() {
    assert!(temperature_settled(&[290.0, 290.1, 290.0, 290.0, 290.0], 0.2).unwrap());
}

#[test]
fn oscillation_is_not_settled() {
    assert!(!temperature_settled(&[290.0, 291.0, 290.0, 291.0, 290.0], 0.2).unwrap());
}
#[test]
fn incomplete_window_is_rejected() {
    assert!(temperature_settled(&[290.0; 4], 0.2).is_err());
}
#[test]
fn invalid_temperature_does_not_settle() {
    assert!(temperature_settled(&[290.0, 290.0, 290.0, 290.0, f64::NAN], 0.2).is_err());
}

#[test]
fn diagnostic_extraction_preserves_the_coolant_api() {
    let samples = [320.0, 320.0, 320.0];
    let original = odyssey_flight::thermal::coolant::cooldown_stalled(&samples, 5.0);
    let extracted = odyssey_flight::thermal::thermal::cooldown_stalled(&samples, 5.0);
    assert_eq!(original, extracted);
    assert_eq!(extracted.unwrap(), true);
}
