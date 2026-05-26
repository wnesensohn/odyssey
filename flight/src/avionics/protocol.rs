use crate::ControlError;

pub const MAGIC: [u8; 2] = *b"OD";
pub const WIRE_VERSION: u8 = 4;
pub const MAX_PAYLOAD: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub kind: u8,
    pub sequence: u32,
    pub payload: Vec<u8>,
}

pub fn checksum(bytes: &[u8]) -> u16 {
    bytes.iter().fold(0xffffu16, |crc, byte| {
        let mut next = crc ^ ((*byte as u16) << 8);
        for _ in 0..8 {
            next = if next & 0x8000 != 0 {
                (next << 1) ^ 0x1021
            } else {
                next << 1
            };
        }
        next
    })
}

pub fn encode(frame: &Frame) -> Result<Vec<u8>, ControlError> {
    if !valid_kind(frame.kind) {
        return Err(ControlError::InvalidFrame);
    }
    if frame.payload.len() > MAX_PAYLOAD {
        return Err(ControlError::CapacityExceeded);
    }
    let mut bytes = Vec::with_capacity(frame.payload.len() + 12);
    bytes.extend_from_slice(&MAGIC);
    bytes.push(WIRE_VERSION);
    bytes.push(frame.kind);
    bytes.extend_from_slice(&frame.sequence.to_be_bytes());
    bytes.extend_from_slice(&(frame.payload.len() as u16).to_be_bytes());
    bytes.extend_from_slice(&frame.payload);
    bytes.extend_from_slice(&checksum(&bytes).to_be_bytes());
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<Frame, ControlError> {
    if bytes.len() < 12 || !valid_kind(bytes[3]) || bytes[..2] != MAGIC || bytes[2] != WIRE_VERSION
    {
        return Err(ControlError::InvalidFrame);
    }
    let length = u16::from_be_bytes([bytes[8], bytes[9]]) as usize;
    if length > MAX_PAYLOAD || bytes.len() != length + 12 {
        return Err(ControlError::InvalidFrame);
    }
    let expected = u16::from_be_bytes([bytes[bytes.len() - 2], bytes[bytes.len() - 1]]);
    if checksum(&bytes[..bytes.len() - 2]) != expected {
        return Err(ControlError::InvalidFrame);
    }
    Ok(Frame {
        kind: bytes[3],
        sequence: u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        payload: bytes[10..10 + length].to_vec(),
    })
}

pub fn valid_kind(kind: u8) -> bool {
    matches!(kind, 1..=3)
}
