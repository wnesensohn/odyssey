use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandKind {
    Safe,
    ArmEngine,
    SetThrottle,
    StopEngine,
    SetHeater,
    AcknowledgeFault,
}

#[derive(Debug, Clone, Copy)]
pub struct Command {
    pub kind: CommandKind,
    pub value: f64,
    pub issued_ms: u64,
    pub expires_ms: u64,
}

impl Command {
    pub fn validate(&self, now_ms: u64) -> Result<(), ControlError> {
        if self.issued_ms > now_ms || now_ms > self.expires_ms {
            return Err(ControlError::StaleSample);
        }
        if self.expires_ms - self.issued_ms > 60_000 {
            return Err(ControlError::OutOfRange);
        }
        match self.kind {
            CommandKind::SetThrottle => {
                finite_in_range(self.value, 0.0, 1.0)?;
            }
            CommandKind::SetHeater => {
                finite_in_range(self.value, 0.0, 5000.0)?;
            }
            CommandKind::AcknowledgeFault => {
                finite_in_range(self.value, 0.0, u16::MAX as f64)?;
            }
            _ if self.value != 0.0 => return Err(ControlError::OutOfRange),
            _ => {}
        }
        Ok(())
    }
}
