use crate::ControlError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionPhase {
    Commissioning,
    OrbitHold,
    Transfer,
    Science,
    Recovery,
    Safe,
}

#[derive(Debug, Clone, Copy)]
pub struct MissionHealth {
    pub communications: bool,
    pub navigation: bool,
    pub power_reserve: bool,
    pub propulsion: bool,
    pub critical_fault: bool,
}

#[derive(Debug, Clone)]
pub struct Mission {
    pub phase: MissionPhase,
    pub previous: MissionPhase,
    pub elapsed_ms: u64,
    pub transitions: u64,
}

impl Default for Mission {
    fn default() -> Self {
        Self {
            phase: MissionPhase::Commissioning,
            previous: MissionPhase::Commissioning,
            elapsed_ms: 0,
            transitions: 0,
        }
    }
}

impl Mission {
    pub fn request(
        &mut self,
        next: MissionPhase,
        health: MissionHealth,
    ) -> Result<(), ControlError> {
        if health.critical_fault && next != MissionPhase::Safe {
            return Err(ControlError::InterlockOpen);
        }
        let valid = match (self.phase, next) {
            (_, MissionPhase::Safe) => true,
            (MissionPhase::Commissioning | MissionPhase::Recovery, MissionPhase::OrbitHold) => {
                health.navigation && health.power_reserve
            }
            (MissionPhase::OrbitHold, MissionPhase::Transfer) => {
                health.navigation
                    && health.propulsion
                    && health.power_reserve
                    && health.communications
            }
            (MissionPhase::OrbitHold, MissionPhase::Science) => health.power_reserve,
            (MissionPhase::Transfer | MissionPhase::Science, MissionPhase::OrbitHold) => {
                health.navigation
            }
            (MissionPhase::Safe, MissionPhase::Recovery) => {
                !health.critical_fault && health.communications
            }
            (current, requested) if current == requested => return Ok(()),
            _ => false,
        };
        if !valid {
            return Err(ControlError::InvalidTransition);
        }
        self.previous = self.phase;
        self.phase = next;
        self.elapsed_ms = 0;
        self.transitions = self.transitions.saturating_add(1);
        Ok(())
    }
    pub fn tick(
        &mut self,
        health: MissionHealth,
        dt_ms: u64,
    ) -> Result<MissionPhase, ControlError> {
        if dt_ms == 0 || dt_ms > 1000 {
            return Err(ControlError::OutOfRange);
        }
        self.elapsed_ms = self.elapsed_ms.saturating_add(dt_ms);
        if health.critical_fault || (!health.power_reserve && self.phase == MissionPhase::Transfer)
        {
            self.request(MissionPhase::Safe, health)?;
        }
        Ok(self.phase)
    }
}
