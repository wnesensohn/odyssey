use odyssey_flight::avionics::{
    drivers::{CanFrame, RegisterBus, SimulatedRegisters},
    mission::{Mission, MissionHealth, MissionPhase},
};
use odyssey_flight::guidance::estimator::ScalarEstimator;
use odyssey_flight::power::breaker::{BreakerState, CircuitBreaker};
use odyssey_flight::propulsion::feed::{FeedObservation, FeedState, FeedSystem};
use odyssey_flight::{engine::EngineState, ControlError};

fn healthy() -> MissionHealth {
    MissionHealth {
        communications: true,
        navigation: true,
        power_reserve: true,
        propulsion: true,
        critical_fault: false,
    }
}
fn feed_observation() -> FeedObservation {
    FeedObservation {
        pressure_kpa: 300.0,
        pump_current_a: 5.0,
        temperature_k: 500.0,
        inlet_open: true,
        inlet_closed: false,
        outlet_open: false,
        outlet_closed: true,
        flame_present: false,
    }
}

#[test]
fn feed_sequence_requires_ready_pressure_before_ignition() {
    let mut feed = FeedSystem::new();
    assert_eq!(feed.ignite(0.3), Err(ControlError::InterlockOpen));
    feed.prepare().unwrap();
    assert_eq!(
        feed.tick(feed_observation(), 100).unwrap(),
        FeedState::Ready
    );
    feed.ignite(0.3).unwrap();
    let mut input = feed_observation();
    input.outlet_open = true;
    input.outlet_closed = false;
    feed.tick(input, 100).unwrap();
    input.flame_present = true;
    feed.tick(input, 100).unwrap();
    assert_eq!(feed.engine.state(), EngineState::Running);
}
#[test]
fn feed_valve_fault_locks_and_stops_the_pump() {
    let mut feed = FeedSystem::new();
    feed.prepare().unwrap();
    let mut input = feed_observation();
    input.inlet_closed = true;
    assert!(feed.tick(input, 100).is_err());
    assert_eq!(feed.state, FeedState::Locked);
    assert_eq!(feed.pump.duty, 0.0);
}
#[test]
fn breaker_instaneous_and_thermal_trips_require_explicit_reset() {
    let mut breaker = CircuitBreaker::new(5.0).unwrap();
    breaker.close().unwrap();
    assert_eq!(breaker.update(16.0, 0.1).unwrap(), BreakerState::Tripped);
    assert_eq!(breaker.close(), Err(ControlError::InterlockOpen));
    assert!(breaker.reset(1.0).is_err());
    breaker.reset(0.0).unwrap();
    breaker.close().unwrap();
    for _ in 0..100 {
        breaker.update(8.0, 0.1).unwrap();
    }
    assert_eq!(breaker.state, BreakerState::Tripped);
}
#[test]
fn estimator_updates_uncertainty_and_rejects_large_innovations() {
    let mut estimator = ScalarEstimator::new(0.0, 1.0).unwrap();
    estimator.predict(1.0, 1.0).unwrap();
    assert_eq!(estimator.estimate, 1.0);
    let before = estimator.variance;
    assert!(estimator.observe(1.1, 0.1).unwrap());
    assert!(estimator.variance < before);
    let before = estimator.estimate;
    assert!(!estimator.observe(100.0, 0.1).unwrap());
    assert_eq!(estimator.estimate, before);
    assert_eq!(estimator.rejected, 1);
}
#[test]
fn mission_transfer_requires_all_prerequisites() {
    let mut mission = Mission::default();
    mission.request(MissionPhase::OrbitHold, healthy()).unwrap();
    let degraded = MissionHealth {
        communications: false,
        ..healthy()
    };
    assert!(mission.request(MissionPhase::Transfer, degraded).is_err());
    mission.request(MissionPhase::Transfer, healthy()).unwrap();
    mission
        .tick(
            MissionHealth {
                power_reserve: false,
                ..healthy()
            },
            100,
        )
        .unwrap();
    assert_eq!(mission.phase, MissionPhase::Safe);
}
#[test]
fn recovery_requires_clear_faults_and_ground_contact() {
    let mut mission = Mission::default();
    mission.request(MissionPhase::Safe, healthy()).unwrap();
    assert!(mission
        .request(
            MissionPhase::Recovery,
            MissionHealth {
                critical_fault: true,
                ..healthy()
            }
        )
        .is_err());
    mission.request(MissionPhase::Recovery, healthy()).unwrap();
    mission.request(MissionPhase::OrbitHold, healthy()).unwrap();
    assert_eq!(mission.transitions, 3);
}
#[test]
fn register_bus_faults_do_not_corrupt_other_addresses() {
    let mut bus = SimulatedRegisters::default();
    bus.write_register(1, 3, 42).unwrap();
    bus.unavailable[2] = true;
    assert_eq!(bus.read_register(2, 3), Err(ControlError::InterlockOpen));
    assert_eq!(bus.read_register(1, 3).unwrap(), 42);
    assert_eq!(bus.write_register(8, 0, 0), Err(ControlError::OutOfRange));
}
#[test]
fn can_frames_preserve_standard_identifier_and_payload() {
    for length in 0..=8 {
        let frame = CanFrame::new(0x321, &vec![0x5a; length]).unwrap();
        assert_eq!(CanFrame::decode(frame.serialize().unwrap()).unwrap(), frame);
    }
    assert!(CanFrame::new(0x800, &[]).is_err());
    assert!(CanFrame::new(1, &[0; 9]).is_err());
}
