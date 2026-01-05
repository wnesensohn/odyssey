use crate::{sensor::Sample, ControlError};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: u16,
    pub unit: String,
    pub samples: VecDeque<Sample>,
    pub capacity: usize,
}

impl Channel {
    pub fn new(id: u16, unit: &str, capacity: usize) -> Result<Self, ControlError> {
        if capacity == 0 || capacity > 4096 || unit.is_empty() {
            return Err(ControlError::OutOfRange);
        }
        Ok(Self {
            id,
            unit: unit.to_owned(),
            samples: VecDeque::new(),
            capacity,
        })
    }
    pub fn append(&mut self, sample: Sample) -> Result<(), ControlError> {
        if !sample.value.is_finite() {
            return Err(ControlError::InvalidSample);
        }
        if self
            .samples
            .back()
            .is_some_and(|last| sample.timestamp_ms < last.timestamp_ms)
        {
            return Err(ControlError::InvalidTransition);
        }
        if self.samples.len() == self.capacity {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);
        Ok(())
    }
    pub fn latest(&self) -> Option<Sample> {
        self.samples.back().copied()
    }
    pub fn mean(&self) -> Option<f64> {
        if self.samples.is_empty() {
            None
        } else {
            Some(self.samples.iter().map(|s| s.value).sum::<f64>() / self.samples.len() as f64)
        }
    }
}
