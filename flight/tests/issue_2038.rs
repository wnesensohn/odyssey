use odyssey_flight::propulsion::valve::*;

#[test]
fn nominal_behavior() {
    assert_eq!(
        debounced_position(&[(true, false); 3], 3).unwrap(),
        Some(true)
    );
}

#[test]
fn boundary_behavior() {
    assert_eq!(
        debounced_position(&[(true, false), (false, false)], 2).unwrap(),
        None
    );
}

#[test]
fn invalid_behavior() {
    assert!(debounced_position(&[(true, true)], 1).is_err());
}
