use odyssey_flight::power::power::*;

#[test]
fn nominal_behavior() {
    assert!(unsupplied_loads(100.0, &[]).unwrap().is_empty());
}
