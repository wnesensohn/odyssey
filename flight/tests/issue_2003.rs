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
