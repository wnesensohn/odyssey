use crate::ControlError;

#[derive(Debug, Clone)]
pub struct Watchdog {
    pub deadline_ms: u64,
    pub timeout_ms: u64,
    pub misses: u32,
    pub armed: bool,
}

impl Watchdog {
    pub fn new(timeout_ms: u64) -> Result<Self, ControlError> {
        if timeout_ms == 0 || timeout_ms > 60_000 {
            return Err(ControlError::OutOfRange);
        }
        Ok(Self {
            deadline_ms: 0,
            timeout_ms,
            misses: 0,
            armed: false,
        })
    }
    pub fn feed(&mut self, now_ms: u64) {
        self.deadline_ms = now_ms.saturating_add(self.timeout_ms);
        self.armed = true;
    }
    pub fn expired(&mut self, now_ms: u64) -> bool {
        if self.armed && now_ms > self.deadline_ms {
            self.misses = self.misses.saturating_add(1);
            self.armed = false;
            return true;
        }
        false
    }
}

pub fn recovery_quorum(healthy: &[bool], minimum: usize) -> Result<bool, crate::ControlError> {
    if minimum == 0 || minimum > healthy.len() {
        return Err(crate::ControlError::OutOfRange);
    }
    Ok(healthy.iter().filter(|&&value| value).count() >= minimum)
}
