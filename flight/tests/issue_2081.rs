use odyssey_flight::power::battery::*;

#[test]
fn nominal_behavior() {
    assert_eq!(charge_acceptance_fraction(293.15).unwrap(), 1.0);
}
