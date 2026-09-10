use odyssey_flight::thermal::coolant::temperature_settled;
#[test]
fn five_consistent_samples_are_settled() {
    assert!(temperature_settled(&[290.0, 290.1, 290.0, 290.0, 290.0], 0.2).unwrap());
}
