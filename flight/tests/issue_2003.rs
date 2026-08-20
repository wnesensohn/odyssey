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
fn marginal_pressure_does_not_enable_ignition() {
    assert!(!fresh_pressure_permits_ignition(
        crate_sample(252.0, 100),
        200,
        250.0,
        500
    ));
}

#[test]
fn exact_margin_permits_ignition() {
    assert!(fresh_pressure_permits_ignition(
        crate_sample(255.0, 100),
        200,
        250.0,
        500
    ));
}

#[test]
fn pressure_decay_closes_the_ignition_interlock() {
    let values = [300.0, 300.0, 260.0, 240.0, 200.0];
    let mut seen_closed = false;
    for (index, value) in values.iter().copied().enumerate() {
        let time = index as u64 * 100;
        let permitted =
            fresh_pressure_permits_ignition(crate_sample(value, time), time, 250.0, 500);
        if !permitted {
            seen_closed = true;
        }
        if seen_closed {
            assert!(!permitted);
        }
    }
    assert!(seen_closed);
}
