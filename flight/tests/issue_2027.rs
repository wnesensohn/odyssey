use odyssey_flight::guidance::attitude::*;
fn q_identity() -> Quaternion {
    Quaternion {
        w: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

#[test]
fn nominal_behavior() {
    assert_eq!(
        orientation_error_rad(q_identity(), q_identity()).unwrap(),
        0.0
    );
}

#[test]
fn boundary_behavior() {
    assert_eq!(
        orientation_error_rad(
            q_identity(),
            Quaternion {
                w: -1.0,
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        )
        .unwrap(),
        0.0
    );
}

#[test]
fn invalid_behavior() {
    assert!(orientation_error_rad(
        q_identity(),
        Quaternion {
            w: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    )
    .is_err());
}
