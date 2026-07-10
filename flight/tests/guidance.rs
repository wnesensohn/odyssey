use odyssey_flight::guidance::{
    shaping::PanelController,
    trajectory::{circular_transfer, propellant_required},
};
use odyssey_flight::{
    attitude::{Quaternion, RateController, Vector3},
    filter::{median, LowPass},
    navigation::{OrbitalState, EARTH_MU},
    ControlError,
};

#[test]
fn vector_basis_cross_products_preserve_handedness() {
    let x = Vector3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let y = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    assert_eq!(
        x.cross(y),
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0
        }
    );
    assert_eq!(x.dot(y), 0.0);
    assert!(Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0
    }
    .normalized()
    .is_err());
}

#[test]
fn quaternion_identity_and_half_turn_rotate_vectors() {
    let point = Vector3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    assert_eq!(
        Quaternion {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
        .rotate(point)
        .unwrap(),
        point
    );
    let rotated = Quaternion {
        w: 0.0,
        x: 0.0,
        y: 0.0,
        z: 1.0,
    }
    .rotate(point)
    .unwrap();
    assert!((rotated.x + 1.0).abs() < 1e-9);
    assert!(rotated.y.abs() < 1e-9);
}

#[test]
fn torque_limit_caps_the_vector_norm() {
    let controller = RateController {
        gain: 2.0,
        maximum_torque_nm: 0.5,
    };
    let torque = controller
        .torque(
            Vector3 {
                x: 10.0,
                y: 10.0,
                z: 0.0,
            },
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        )
        .unwrap();
    assert!((torque.norm() - 0.5).abs() < 1e-9);
}

#[test]
fn circular_orbit_has_negative_specific_energy() {
    let radius = 7_000_000.0;
    let orbit = OrbitalState {
        position_m: Vector3 {
            x: radius,
            y: 0.0,
            z: 0.0,
        },
        velocity_mps: Vector3 {
            x: 0.0,
            y: (EARTH_MU / radius).sqrt(),
            z: 0.0,
        },
        timestamp_ms: 0,
    };
    assert!((orbit.specific_energy().unwrap() + EARTH_MU / (2.0 * radius)).abs() < 1e-6);
    assert!(orbit.propagate(0.1).unwrap().position_m.y > 0.0);
    assert!(orbit.propagate(100.0).is_err());
}

#[test]
fn smoothing_and_median_reject_invalid_input() {
    let mut filter = LowPass::new(1.0).unwrap();
    assert_eq!(filter.update(10.0, 0.1).unwrap(), 10.0);
    let value = filter.update(20.0, 0.1).unwrap();
    assert!(value > 10.0 && value < 20.0);
    assert_eq!(median(&[3.0, 1.0, 2.0]).unwrap(), 2.0);
    assert_eq!(median(&[]), Err(ControlError::InvalidSample));
}

#[test]
fn panel_actuation_is_bounded_and_stops_at_target() {
    let mut panel = PanelController::default();
    panel.target(0.5).unwrap();
    for _ in 0..1000 {
        panel.tick(0.1, true).unwrap();
        assert!(panel.angular_rate_rps.abs() <= 0.05 + 1e-9);
    }
    assert!((panel.angle_rad - 0.5).abs() < 0.01);
    assert_eq!(panel.tick(0.1, false), Err(ControlError::InterlockOpen));
}

#[test]
fn orbital_transfer_and_mass_budget_have_physical_signs() {
    let transfer = circular_transfer(7e6, 8e6).unwrap();
    assert!(
        transfer.first_burn_mps > 0.0 && transfer.second_burn_mps > 0.0 && transfer.coast_s > 0.0
    );
    assert_eq!(propellant_required(1000.0, 0.0, 300.0).unwrap(), 0.0);
    assert!(propellant_required(1000.0, 100.0, 300.0).unwrap() > 0.0);
}

#[test]
fn reset_filter_discards_previous_samples() {
    let mut filter = odyssey_flight::filter::LowPass::new(1.0).unwrap();
    filter.update(100.0, 1.0).unwrap();
    filter.reset();
    assert_eq!(filter.update(2.0, 1.0).unwrap(), 2.0);
}
