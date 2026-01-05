use crate::{protocol::checksum, ControlError};

#[derive(Debug, Clone)]
pub struct Record {
    pub timestamp_ms: u64,
    pub channel: u16,
    pub data: Vec<u8>,
    pub checksum: u16,
}

#[derive(Debug)]
pub struct Recorder {
    records: Vec<Record>,
    capacity: usize,
}

impl Recorder {
    pub fn new(capacity: usize) -> Result<Self, ControlError> {
        if capacity == 0 || capacity > 100_000 {
            return Err(ControlError::OutOfRange);
        }
        Ok(Self {
            records: Vec::new(),
            capacity,
        })
    }
    pub fn append(
        &mut self,
        timestamp_ms: u64,
        channel: u16,
        data: &[u8],
    ) -> Result<(), ControlError> {
        if data.len() > 1024 || self.records.len() == self.capacity {
            return Err(ControlError::CapacityExceeded);
        }
        if self
            .records
            .last()
            .is_some_and(|last| timestamp_ms < last.timestamp_ms)
        {
            return Err(ControlError::InvalidTransition);
        }
        self.records.push(Record {
            timestamp_ms,
            channel,
            data: data.to_vec(),
            checksum: checksum(data),
        });
        Ok(())
    }
    pub fn replay(&self, channel: u16, start_ms: u64) -> Result<Vec<&Record>, ControlError> {
        let mut result = Vec::new();
        for record in &self.records {
            if record.channel == channel && record.timestamp_ms >= start_ms {
                if checksum(&record.data) != record.checksum {
                    return Err(ControlError::InvalidFrame);
                }
                result.push(record);
            }
        }
        Ok(result)
    }
}
