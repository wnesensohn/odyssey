use crate::ControlError;

#[derive(Debug, Clone)]
pub struct MissionClock {
    monotonic_ms: u64,
    offset_ms: i64,
    synchronized: bool,
}

impl Default for MissionClock {
    fn default() -> Self {
        Self {
            monotonic_ms: 0,
            offset_ms: 0,
            synchronized: false,
        }
    }
}

impl MissionClock {
    pub fn advance(&mut self, next_ms: u64) -> Result<(), ControlError> {
        if next_ms < self.monotonic_ms {
            return Err(ControlError::InvalidTransition);
        }
        self.monotonic_ms = next_ms;
        Ok(())
    }
    pub fn synchronize(
        &mut self,
        ground_ms: u64,
        maximum_step_ms: u64,
    ) -> Result<(), ControlError> {
        let difference = ground_ms as i128 - self.monotonic_ms as i128;
        if difference.abs() > maximum_step_ms as i128
            || difference > i64::MAX as i128
            || difference < i64::MIN as i128
        {
            return Err(ControlError::OutOfRange);
        }
        self.offset_ms = difference as i64;
        self.synchronized = true;
        Ok(())
    }
    pub fn epoch_ms(&self) -> Option<u64> {
        if !self.synchronized {
            return None;
        }
        u64::try_from(self.monotonic_ms as i128 + self.offset_ms as i128).ok()
    }
    pub fn monotonic_ms(&self) -> u64 {
        self.monotonic_ms
    }
}
