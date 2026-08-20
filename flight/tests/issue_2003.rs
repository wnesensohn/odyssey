use odyssey_flight::propulsion::pressure::*;
fn crate_sample(value: f64, timestamp_ms: u64) -> odyssey_flight::sensor::Sample {
    odyssey_flight::sensor::Sample {
        value,
        timestamp_ms,
        quality: odyssey_flight::sensor::Quality::Good,
    }
}

#[test]
fn nominal_behavior() {
    assert!(fresh_pressure_permits_ignition(
        crate_sample(300.0, 100),
        200,
        250.0,
        500
    ));
}

#[test]
fn boundary_behavior() {
    assert!(!fresh_pressure_permits_ignition(
        crate_sample(300.0, 100),
        601,
        250.0,
        500
    ));
}

#[test]
fn invalid_behavior() {
    assert!(!fresh_pressure_permits_ignition(
        crate_sample(f64::NAN, 100),
        200,
        250.0,
        500
    ));
}

#[test]
fn adaptive_margin_requires_more_than_ten_kpa() {
    assert!(!fresh_pressure_permits_ignition(
        crate_sample(260.0, 100),
        200,
        250.0,
        500
    ));
    assert!(fresh_pressure_permits_ignition(
        crate_sample(270.0, 100),
        200,
        250.0,
        500
    ));
}

#[test]
fn hot_chamber_requires_additional_feed_pressure() {
    assert_eq!(required_feed_kpa(250.0, 800.0).unwrap(), 285.0);
    assert!(required_feed_kpa(250.0, f64::NAN).is_err());
}
