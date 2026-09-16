use odyssey_flight::propulsion::tank::Tank;
use odyssey_flight::{
    engine::{EngineController, EngineInput, EngineState},
    pressure::{pressure_permits_ignition, PressureWindow},
    pump::{PumpController, PumpState},
    valve::{Valve, ValvePosition},
    ControlError,
};

fn nominal() -> EngineInput {
    EngineInput {
        feed_pressure_kpa: 300.0,
        chamber_temperature_k: 500.0,
        valves_ready: true,
        ignition_confirmed: true,
        stop_requested: false,
    }
}

#[test]
fn engine_requires_priming_before_ignition() {
    let mut engine = EngineController::default();
    assert_eq!(engine.state(), EngineState::Safe);
    engine.arm().unwrap();
    assert_eq!(engine.tick(nominal(), 100).unwrap(), EngineState::Igniting);
    assert_eq!(engine.tick(nominal(), 100).unwrap(), EngineState::Running);
}

#[test]
fn running_throttle_changes_are_slew_limited() {
    let mut engine = EngineController::default();
    engine.arm().unwrap();
    engine.tick(nominal(), 100).unwrap();
    engine.tick(nominal(), 100).unwrap();
    engine.set_throttle(1.0).unwrap();
    engine.tick(nominal(), 100).unwrap();
    assert!((engine.throttle() - 0.02).abs() < 1e-9);
    assert_eq!(
        engine.set_throttle(f64::NAN),
        Err(ControlError::InvalidSample)
    );
}

#[test]
fn overtemperature_latches_safe_zero_throttle() {
    let mut engine = EngineController::default();
    engine.arm().unwrap();
    let mut input = nominal();
    input.chamber_temperature_k = 1300.0;
    assert_eq!(engine.tick(input, 100), Err(ControlError::InterlockOpen));
    assert_eq!(engine.state(), EngineState::Fault);
    assert_eq!(engine.throttle(), 0.0);
    assert!(engine.reset_fault(600.0).is_err());
    engine.reset_fault(300.0).unwrap();
    assert_eq!(engine.state(), EngineState::Safe);
}

#[test]
fn pump_stops_on_excess_current() {
    let mut pump = PumpController::new(300.0).unwrap();
    pump.start().unwrap();
    for _ in 0..20 {
        pump.regulate(250.0, 5.0, 0.1).unwrap();
    }
    assert!(pump.duty > 0.0 && pump.duty <= 1.0);
    assert_eq!(
        pump.regulate(300.0, 20.0, 0.1),
        Err(ControlError::InterlockOpen)
    );
    assert_eq!(pump.state, PumpState::Tripped);
    assert_eq!(pump.duty, 0.0);
}

#[test]
fn valve_detects_inconsistent_switches_and_timeouts() {
    let mut valve = Valve::default();
    valve.command(true).unwrap();
    assert_eq!(
        valve.tick(true, true, 100),
        Err(ControlError::InvalidSample)
    );
    assert_eq!(valve.position, ValvePosition::Jammed);
    let mut valve = Valve::default();
    valve.command(true).unwrap();
    assert!(valve.tick(false, false, 1600).is_err());
}

#[test]
fn pressure_window_retains_only_its_capacity() {
    let mut window = PressureWindow::new(3).unwrap();
    for value in [100.0, 200.0, 300.0] {
        window.push(value).unwrap();
    }
    assert_eq!(window.push(400.0).unwrap(), 300.0);
    assert_eq!(window.spread(), 200.0);
    assert!(!pressure_permits_ignition(f64::NAN, 200.0));
    assert!(pressure_permits_ignition(250.0, 250.0));
}

#[test]
fn tank_consumption_cannot_create_negative_inventory() {
    let mut tank = Tank {
        capacity_kg: 100.0,
        remaining_kg: 50.0,
        temperature_k: 290.0,
        ullage_m3: 1.0,
        pressurant_mol: 100.0,
    };
    assert!(tank.pressure_kpa().unwrap() > 200.0);
    assert_eq!(tank.consume(2.0, 10.0).unwrap(), 30.0);
    assert!(tank.consume(10.0, 10.0).is_err());
    assert_eq!(tank.remaining_kg, 30.0);
}

#[test]
fn empty_hardware_capacity_is_not_a_valid_tank() {
    let mut tank = odyssey_flight::propulsion::tank::Tank {
        capacity_kg: 0.0,
        remaining_kg: 0.0,
        temperature_k: 290.0,
        ullage_m3: 1.0,
        pressurant_mol: 1.0,
    };
    assert!(tank.consume(0.0, 1.0).is_err());
    assert_eq!(tank.remaining_kg, 0.0);
}

#[test]
fn repeated_valve_commands_do_not_restart_movement_timeout() {
    use odyssey_flight::propulsion::valve::{Valve, ValvePosition};
    let mut valve = Valve::default();
    valve.command(true).unwrap();
    valve.tick(false, false, 1000).unwrap();
    valve.command(true).unwrap();
    assert!(valve.tick(false, false, 600).is_err());
    assert_eq!(valve.position, ValvePosition::Jammed);
}
