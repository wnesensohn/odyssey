use odyssey_flight::avionics::{scheduler::Scheduler, storage::Recorder};
use odyssey_flight::{
    actuator::{ActuatorBus, SimulatedBus},
    command::{Command, CommandKind},
    fault::{FaultRegistry, Severity},
    sensor::{Calibration, Quality, Sample, SensorLimits},
    telemetry::Channel,
    time::MissionClock,
    watchdog::Watchdog,
    ControlError,
};

#[test]
fn stale_and_future_sensor_samples_are_distinct_errors() {
    let limits = SensorLimits {
        minimum: 0.0,
        maximum: 100.0,
        maximum_age_ms: 500,
    };
    let sample = Sample {
        value: 25.0,
        timestamp_ms: 100,
        quality: Quality::Good,
    };
    assert_eq!(limits.validate(sample, 601), Err(ControlError::StaleSample));
    assert_eq!(
        limits.validate(sample, 99),
        Err(ControlError::InvalidSample)
    );
    assert_eq!(limits.validate(sample, 600).unwrap(), 25.0);
}

#[test]
fn calibration_applies_affine_mapping() {
    let calibration = Calibration {
        gain: 0.5,
        offset: 10.0,
    };
    assert_eq!(calibration.apply(20.0).unwrap(), 20.0);
    assert_eq!(
        calibration.apply(f64::INFINITY),
        Err(ControlError::InvalidSample)
    );
}

#[test]
fn watchdog_counts_one_miss_per_arming() {
    let mut watchdog = Watchdog::new(1000).unwrap();
    watchdog.feed(0);
    assert!(!watchdog.expired(1000));
    assert!(watchdog.expired(1001));
    assert!(!watchdog.expired(2000));
    assert_eq!(watchdog.misses, 1);
}

#[test]
fn fault_acknowledgement_does_not_clear_critical_state() {
    let mut registry = FaultRegistry::default();
    registry.report(12, Severity::Critical, 100);
    registry.report(12, Severity::Advisory, 200);
    assert!(registry.acknowledge(12));
    assert!(registry.safe_mode_required());
    let fault = registry.clear(12).unwrap();
    assert_eq!(fault.occurrences, 2);
    assert!(!registry.safe_mode_required());
}

#[test]
fn simulated_bus_can_inject_and_clear_failures() {
    let mut bus = SimulatedBus::default();
    bus.write_duty(3, 0.5).unwrap();
    assert_eq!(bus.read_current(3).unwrap(), 6.0);
    bus.inject_failure(3).unwrap();
    assert_eq!(bus.write_duty(3, 0.6), Err(ControlError::InterlockOpen));
    bus.clear_failures();
    bus.write_duty(3, 0.0).unwrap();
}

#[test]
fn command_lifetime_and_values_are_checked() {
    let command = Command {
        kind: CommandKind::SetThrottle,
        value: 0.5,
        issued_ms: 100,
        expires_ms: 1000,
    };
    assert!(command.validate(500).is_ok());
    assert_eq!(command.validate(1001), Err(ControlError::StaleSample));
    assert_eq!(
        Command {
            value: 1.1,
            ..command
        }
        .validate(500),
        Err(ControlError::OutOfRange)
    );
}

#[test]
fn bounded_telemetry_discards_oldest_without_reordering() {
    let mut channel = Channel::new(3, "kPa", 2).unwrap();
    for i in 0..3 {
        channel
            .append(Sample {
                value: i as f64,
                timestamp_ms: i,
                quality: Quality::Good,
            })
            .unwrap();
    }
    assert_eq!(channel.samples.len(), 2);
    assert_eq!(channel.latest().unwrap().timestamp_ms, 2);
    assert_eq!(channel.mean().unwrap(), 1.5);
}

#[test]
fn clock_synchronization_never_rewinds_monotonic_time() {
    let mut clock = MissionClock::default();
    clock.advance(1000).unwrap();
    clock.synchronize(1100, 200).unwrap();
    assert_eq!(clock.epoch_ms(), Some(1100));
    assert_eq!(clock.advance(999), Err(ControlError::InvalidTransition));
}

#[test]
fn scheduler_enforces_unique_ids_and_execution_budgets() {
    let mut scheduler = Scheduler::default();
    scheduler.add(1, 100, 1000).unwrap();
    assert_eq!(scheduler.due(0), vec![1]);
    assert!(scheduler.due(99).is_empty());
    assert_eq!(scheduler.due(100), vec![1]);
    assert!(scheduler.complete(1, 1001).unwrap());
    assert_eq!(scheduler.tasks()[0].overruns, 1);
    assert!(scheduler.add(1, 100, 1000).is_err());
}

#[test]
fn recorder_filters_channels_and_checks_capacity() {
    let mut recorder = Recorder::new(2).unwrap();
    recorder.append(100, 1, b"nominal").unwrap();
    recorder.append(200, 2, b"degraded").unwrap();
    assert_eq!(recorder.replay(1, 0).unwrap().len(), 1);
    assert_eq!(
        recorder.append(300, 3, b"full"),
        Err(ControlError::CapacityExceeded)
    );
}
