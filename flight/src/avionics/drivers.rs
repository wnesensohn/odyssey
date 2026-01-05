use crate::ControlError;

pub trait RegisterBus {
    fn read_register(&mut self, address: u8, register: u8) -> Result<u16, ControlError>;
    fn write_register(&mut self, address: u8, register: u8, value: u16)
        -> Result<(), ControlError>;
}

#[derive(Debug, Clone)]
pub struct SimulatedRegisters {
    pub registers: [[u16; 16]; 8],
    pub unavailable: [bool; 8],
}

impl Default for SimulatedRegisters {
    fn default() -> Self {
        Self {
            registers: [[0; 16]; 8],
            unavailable: [false; 8],
        }
    }
}

impl RegisterBus for SimulatedRegisters {
    fn read_register(&mut self, address: u8, register: u8) -> Result<u16, ControlError> {
        if address >= 8 || register >= 16 {
            return Err(ControlError::OutOfRange);
        }
        if self.unavailable[address as usize] {
            return Err(ControlError::InterlockOpen);
        }
        Ok(self.registers[address as usize][register as usize])
    }
    fn write_register(
        &mut self,
        address: u8,
        register: u8,
        value: u16,
    ) -> Result<(), ControlError> {
        self.read_register(address, register)?;
        self.registers[address as usize][register as usize] = value;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanFrame {
    pub identifier: u16,
    pub length: u8,
    pub payload: [u8; 8],
}

impl CanFrame {
    pub fn new(identifier: u16, payload: &[u8]) -> Result<Self, ControlError> {
        if identifier > 0x7ff || payload.len() > 8 {
            return Err(ControlError::OutOfRange);
        }
        let mut bytes = [0; 8];
        bytes[..payload.len()].copy_from_slice(payload);
        Ok(Self {
            identifier,
            length: payload.len() as u8,
            payload: bytes,
        })
    }
    pub fn serialize(&self) -> Result<[u8; 11], ControlError> {
        if self.identifier > 0x7ff || self.length > 8 {
            return Err(ControlError::InvalidFrame);
        }
        let mut bytes = [0; 11];
        bytes[..2].copy_from_slice(&self.identifier.to_be_bytes());
        bytes[2] = self.length;
        bytes[3..].copy_from_slice(&self.payload);
        Ok(bytes)
    }
    pub fn decode(bytes: [u8; 11]) -> Result<Self, ControlError> {
        let identifier = u16::from_be_bytes([bytes[0], bytes[1]]);
        let length = bytes[2];
        if identifier > 0x7ff || length > 8 {
            return Err(ControlError::InvalidFrame);
        }
        let mut payload = [0; 8];
        payload.copy_from_slice(&bytes[3..]);
        Ok(Self {
            identifier,
            length,
            payload,
        })
    }
}
