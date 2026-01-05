use crate::{
    engine::{EngineController, EngineInput, EngineState},
    pump::PumpController,
    valve::{Valve, ValvePosition},
    ControlError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedState {
    Isolated,
    Pressurizing,
    Ready,
    Burning,
    Purging,
    Locked,
}

#[derive(Debug, Clone)]
pub struct FeedSystem {
    pub state: FeedState,
    pub pump: PumpController,
    pub inlet: Valve,
    pub outlet: Valve,
    pub engine: EngineController,
    elapsed_ms: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct FeedObservation {
    pub pressure_kpa: f64,
    pub pump_current_a: f64,
    pub temperature_k: f64,
    pub inlet_open: bool,
    pub inlet_closed: bool,
    pub outlet_open: bool,
    pub outlet_closed: bool,
    pub flame_present: bool,
}

impl FeedSystem {
    pub fn new() -> Self {
        Self {
            state: FeedState::Isolated,
            pump: PumpController::new(300.0).expect("valid nominal pressure"),
            inlet: Valve::default(),
            outlet: Valve::default(),
            engine: EngineController::default(),
            elapsed_ms: 0,
        }
    }
    pub fn prepare(&mut self) -> Result<(), ControlError> {
        if self.state != FeedState::Isolated {
            return Err(ControlError::InvalidTransition);
        }
        self.inlet.command(true)?;
        self.pump.start()?;
        self.state = FeedState::Pressurizing;
        self.elapsed_ms = 0;
        Ok(())
    }
    pub fn ignite(&mut self, throttle: f64) -> Result<(), ControlError> {
        if self.state != FeedState::Ready {
            return Err(ControlError::InterlockOpen);
        }
        self.engine.set_throttle(throttle)?;
        self.engine.arm()?;
        self.outlet.command(true)?;
        self.state = FeedState::Burning;
        self.elapsed_ms = 0;
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), ControlError> {
        if !matches!(self.state, FeedState::Burning | FeedState::Ready) {
            return Err(ControlError::InvalidTransition);
        }
        self.engine.set_throttle(0.0)?;
        self.outlet.command(false)?;
        self.state = FeedState::Purging;
        self.elapsed_ms = 0;
        Ok(())
    }
    fn lock(&mut self) {
        self.state = FeedState::Locked;
        self.pump.stop();
        let _ = self.outlet.command(false);
        let _ = self.inlet.command(false);
    }
    pub fn tick(
        &mut self,
        observation: FeedObservation,
        dt_ms: u64,
    ) -> Result<FeedState, ControlError> {
        if dt_ms == 0 || dt_ms > 1000 {
            return Err(ControlError::OutOfRange);
        }
        self.elapsed_ms = self.elapsed_ms.saturating_add(dt_ms);
        let inlet = self
            .inlet
            .tick(observation.inlet_open, observation.inlet_closed, dt_ms);
        let outlet = self
            .outlet
            .tick(observation.outlet_open, observation.outlet_closed, dt_ms);
        if inlet.is_err() || outlet.is_err() {
            self.lock();
            return Err(ControlError::InterlockOpen);
        }
        match self.state {
            FeedState::Pressurizing => {
                if self
                    .pump
                    .regulate(
                        observation.pressure_kpa,
                        observation.pump_current_a,
                        dt_ms as f64 / 1000.0,
                    )
                    .is_err()
                {
                    self.lock();
                    return Err(ControlError::InterlockOpen);
                }
                if self.inlet.position == ValvePosition::Open && observation.pressure_kpa >= 250.0 {
                    self.state = FeedState::Ready;
                } else if self.elapsed_ms > 5000 {
                    self.lock();
                    return Err(ControlError::InterlockOpen);
                }
            }
            FeedState::Burning | FeedState::Purging => {
                let input = EngineInput {
                    feed_pressure_kpa: observation.pressure_kpa,
                    chamber_temperature_k: observation.temperature_k,
                    valves_ready: self.outlet.position == ValvePosition::Open,
                    ignition_confirmed: observation.flame_present,
                    stop_requested: self.state == FeedState::Purging,
                };
                if self.engine.tick(input, dt_ms).is_err() {
                    self.lock();
                    return Err(ControlError::InterlockOpen);
                }
                if self.state == FeedState::Purging && self.engine.state() == EngineState::Safe {
                    self.inlet.command(false)?;
                    self.pump.stop();
                    self.state = FeedState::Isolated;
                }
            }
            _ => {}
        }
        Ok(self.state)
    }
}

impl Default for FeedSystem {
    fn default() -> Self {
        Self::new()
    }
}
