use crate::ControlError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValvePosition {
    Closed,
    Opening,
    Open,
    Closing,
    Jammed,
}

#[derive(Debug, Clone)]
pub struct Valve {
    pub position: ValvePosition,
    pub elapsed_ms: u64,
    pub movement_timeout_ms: u64,
}

impl Default for Valve {
    fn default() -> Self {
        Self {
            position: ValvePosition::Closed,
            elapsed_ms: 0,
            movement_timeout_ms: 1500,
        }
    }
}

impl Valve {
    pub fn command(&mut self, open: bool) -> Result<(), ControlError> {
        if self.position == ValvePosition::Jammed {
            return Err(ControlError::InterlockOpen);
        }
        let next = match (self.position, open) {
            (ValvePosition::Closed | ValvePosition::Closing, true) => ValvePosition::Opening,
            (ValvePosition::Open | ValvePosition::Opening, false) => ValvePosition::Closing,
            (position, _) => position,
        };
        if next != self.position {
            self.position = next;
            self.elapsed_ms = 0;
        }
        Ok(())
    }

    pub fn tick(
        &mut self,
        open_switch: bool,
        closed_switch: bool,
        dt_ms: u64,
    ) -> Result<ValvePosition, ControlError> {
        if open_switch && closed_switch {
            self.position = ValvePosition::Jammed;
            return Err(ControlError::InvalidSample);
        }
        self.elapsed_ms = self.elapsed_ms.saturating_add(dt_ms);
        match self.position {
            ValvePosition::Opening if open_switch => self.position = ValvePosition::Open,
            ValvePosition::Closing if closed_switch => self.position = ValvePosition::Closed,
            ValvePosition::Opening | ValvePosition::Closing
                if self.elapsed_ms > self.movement_timeout_ms =>
            {
                self.position = ValvePosition::Jammed;
                return Err(ControlError::InterlockOpen);
            }
            _ => {}
        }
        Ok(self.position)
    }
}

pub fn debounced_position(
    observations: &[(bool, bool)],
    quorum: usize,
) -> Result<Option<bool>, crate::ControlError> {
    if quorum == 0 || quorum > observations.len() {
        return Err(crate::ControlError::OutOfRange);
    }
    if observations.iter().any(|&(open, closed)| open && closed) {
        return Err(crate::ControlError::InvalidSample);
    }
    let tail = &observations[observations.len() - quorum..];
    if tail.iter().all(|&(open, closed)| open && !closed) {
        Ok(Some(true))
    } else if tail.iter().all(|&(open, closed)| !open && closed) {
        Ok(Some(false))
    } else {
        Ok(None)
    }
}
