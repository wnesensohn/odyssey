use odyssey_flight::propulsion::valve::*;

#[test]
fn nominal_behavior() {
    assert_eq!(
        debounced_position(&[(true, false); 3], 3).unwrap(),
        Some(true)
    );
}
