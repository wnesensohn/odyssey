use crate::{finite_in_range, ControlError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn norm(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn scaled(self, scale: f64) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn normalized(self) -> Result<Self, ControlError> {
        let magnitude = self.norm();
        if !magnitude.is_finite() || magnitude < 1e-12 {
            return Err(ControlError::InvalidSample);
        }
        Ok(self.scaled(1.0 / magnitude))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quaternion {
    pub fn normalized(self) -> Result<Self, ControlError> {
        let magnitude =
            (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if !magnitude.is_finite() || magnitude < 1e-12 {
            return Err(ControlError::InvalidSample);
        }
        Ok(Self {
            w: self.w / magnitude,
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        })
    }

    pub fn rotate(self, vector: Vector3) -> Result<Vector3, ControlError> {
        let q = self.normalized()?;
        let axis = Vector3 {
            x: q.x,
            y: q.y,
            z: q.z,
        };
        let first = axis.cross(vector).scaled(2.0);
        let second = axis.cross(first);
        Ok(Vector3 {
            x: vector.x + q.w * first.x + second.x,
            y: vector.y + q.w * first.y + second.y,
            z: vector.z + q.w * first.z + second.z,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RateController {
    pub gain: f64,
    pub maximum_torque_nm: f64,
}

impl RateController {
    pub fn torque(&self, target: Vector3, measured: Vector3) -> Result<Vector3, ControlError> {
        finite_in_range(self.gain, 0.0, 100.0)?;
        finite_in_range(self.maximum_torque_nm, 0.0, 20.0)?;
        let error = Vector3 {
            x: target.x - measured.x,
            y: target.y - measured.y,
            z: target.z - measured.z,
        };
        if !error.norm().is_finite() {
            return Err(ControlError::InvalidSample);
        }
        let mut output = error.scaled(self.gain);
        let norm = output.norm();
        if norm > self.maximum_torque_nm && norm > 0.0 {
            output = output.scaled(self.maximum_torque_nm / norm);
        }
        Ok(output)
    }
}
