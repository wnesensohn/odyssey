use odyssey_flight::guidance::statistics::median;
#[test]
fn median_is_independent_of_input_order() {
    for values in [[1.0, 9.0, 3.0], [9.0, 3.0, 1.0], [3.0, 1.0, 9.0]] {
        assert_eq!(median(&values).unwrap(), 3.0);
    }
}
#[test]
fn repeated_even_samples_preserve_the_midpoint() {
    assert_eq!(median(&[2.0, 4.0, 2.0, 4.0]).unwrap(), 3.0);
}
