use crate::ControlError;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u16,
    pub period_ms: u64,
    pub budget_us: u64,
    pub deadline_ms: u64,
    pub overruns: u64,
}

#[derive(Debug, Default)]
pub struct Scheduler {
    tasks: Vec<Task>,
}

impl Scheduler {
    pub fn add(&mut self, id: u16, period_ms: u64, budget_us: u64) -> Result<(), ControlError> {
        if period_ms == 0 || budget_us == 0 || budget_us > period_ms.saturating_mul(1000) {
            return Err(ControlError::OutOfRange);
        }
        if self.tasks.iter().any(|task| task.id == id) {
            return Err(ControlError::InvalidTransition);
        }
        self.tasks.push(Task {
            id,
            period_ms,
            budget_us,
            deadline_ms: 0,
            overruns: 0,
        });
        Ok(())
    }
    pub fn due(&mut self, now_ms: u64) -> Vec<u16> {
        let mut ready = Vec::new();
        for task in &mut self.tasks {
            if now_ms >= task.deadline_ms {
                ready.push(task.id);
                task.deadline_ms = now_ms.saturating_add(task.period_ms);
            }
        }
        ready
    }
    pub fn complete(&mut self, id: u16, elapsed_us: u64) -> Result<bool, ControlError> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or(ControlError::OutOfRange)?;
        let overrun = elapsed_us > task.budget_us;
        if overrun {
            task.overruns = task.overruns.saturating_add(1);
        }
        Ok(overrun)
    }
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }
}
