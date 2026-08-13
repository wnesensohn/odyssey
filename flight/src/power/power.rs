use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    LifeSupport,
    FlightControl,
    Communications,
    Payload,
}

#[derive(Debug, Clone, Copy)]
pub struct Load {
    pub id: u16,
    pub demand_w: f64,
    pub priority: Priority,
}

pub fn allocate_power(available_w: f64, loads: &[Load]) -> Result<Vec<u16>, ControlError> {
    finite_in_range(available_w, 0.0, 100_000.0)?;
    let mut identifiers = std::collections::BTreeSet::new();
    if loads.iter().any(|load| !identifiers.insert(load.id)) {
        return Err(ControlError::InvalidSample);
    }
    let mut ordered = loads.to_vec();
    for load in &ordered {
        finite_in_range(load.demand_w, 0.0, 20_000.0)?;
    }
    ordered.sort_by_key(|load| (load.priority, load.id));
    let mut remaining = available_w;
    let mut supplied = Vec::new();
    for load in ordered {
        if load.demand_w <= remaining {
            supplied.push(load.id);
            remaining -= load.demand_w;
        }
    }
    Ok(supplied)
}

pub fn solar_output(
    area_m2: f64,
    incidence_rad: f64,
    efficiency: f64,
) -> Result<f64, ControlError> {
    finite_in_range(area_m2, 0.0, 1000.0)?;
    finite_in_range(incidence_rad, -std::f64::consts::PI, std::f64::consts::PI)?;
    finite_in_range(efficiency, 0.0, 1.0)?;
    Ok(area_m2 * 1361.0 * incidence_rad.cos().max(0.0) * efficiency)
}

pub fn unsupplied_loads(available_w: f64, loads: &[Load]) -> Result<Vec<u16>, crate::ControlError> {
    let supplied = allocate_power(available_w, loads)?;
    Ok(loads
        .iter()
        .filter(|load| !supplied.contains(&load.id))
        .map(|load| load.id)
        .collect())
}

pub fn illumination_state(
    incidence_rad: f64,
    eclipse: bool,
) -> Result<&'static str, crate::ControlError> {
    crate::finite_in_range(incidence_rad, -std::f64::consts::PI, std::f64::consts::PI)?;
    Ok(if eclipse {
        "eclipse"
    } else if incidence_rad.cos() <= 0.0 {
        "back-facing"
    } else if incidence_rad.cos() < 0.1 {
        "grazing"
    } else {
        "illuminated"
    })
}

pub fn restore_order(
    loads: &[Load],
    available_w: f64,
    reserve_w: f64,
) -> Result<Vec<u16>, crate::ControlError> {
    crate::finite_in_range(reserve_w, 0.0, available_w)?;
    allocate_power(available_w - reserve_w, loads)
}
