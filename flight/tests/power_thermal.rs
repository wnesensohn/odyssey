use odyssey_flight::thermal::coolant::CoolantLoop;
use odyssey_flight::{
    battery::Battery,
    power::{allocate_power, solar_output, Load, Priority},
    thermal::{radiative_loss, Heater},
};

#[test]
fn flight_control_loads_win_over_payload() {
    let loads = [
        Load {
            id: 1,
            demand_w: 80.0,
            priority: Priority::Payload,
        },
        Load {
            id: 2,
            demand_w: 60.0,
            priority: Priority::FlightControl,
        },
    ];
    assert_eq!(allocate_power(100.0, &loads).unwrap(), vec![2]);
    assert_eq!(allocate_power(150.0, &loads).unwrap(), vec![2, 1]);
}
#[test]
fn solar_panels_do_not_generate_power_from_the_back_side() {
    assert!(solar_output(10.0, 0.0, 0.3).unwrap() > 4000.0);
    assert_eq!(solar_output(10.0, std::f64::consts::PI, 0.3).unwrap(), 0.0);
}
#[test]
fn battery_tracks_charge_and_discharge_efficiency() {
    let mut battery = Battery::new(100.0, 0.5).unwrap();
    assert!(battery.integrate(100.0, 60.0).unwrap() > 0.5);
    assert!(battery.integrate(-200.0, 60.0).unwrap() < 0.5);
    assert!(battery.reserve_available());
}
#[test]
fn heater_hysteresis_retains_state_inside_the_band() {
    let mut heater = Heater::new(280.0, 290.0, 100.0).unwrap();
    assert_eq!(heater.update(279.0, true).unwrap(), 100.0);
    assert_eq!(heater.update(285.0, true).unwrap(), 100.0);
    assert_eq!(heater.update(291.0, true).unwrap(), 0.0);
    assert_eq!(heater.update(279.0, false).unwrap(), 0.0);
}
#[test]
fn heat_balance_has_consistent_direction_and_units() {
    assert_eq!(radiative_loss(1.0, 0.8, 300.0, 300.0).unwrap(), 0.0);
    assert!(radiative_loss(1.0, 0.8, 300.0, 3.0).unwrap() > 0.0);
    let coolant = CoolantLoop {
        inlet_k: 280.0,
        outlet_k: 290.0,
        flow_kgps: 1.0,
        heat_capacity_jkgk: 4000.0,
    };
    assert_eq!(coolant.heat_removed_w().unwrap(), 40_000.0);
    assert_eq!(coolant.required_flow(40_000.0, 10.0).unwrap(), 1.0);
}

#[test]
fn duplicate_load_ids_are_not_an_allocation() {
    use odyssey_flight::power::{allocate_power, Load, Priority};
    let load = Load {
        id: 1,
        demand_w: 10.0,
        priority: Priority::Payload,
    };
    assert!(allocate_power(100.0, &[load, load]).is_err());
}

#[test]
fn empty_allocation_is_valid() {
    assert!(odyssey_flight::power::allocate_power(100.0, &[])
        .unwrap()
        .is_empty());
}
