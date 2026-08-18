use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Advisory,
    Degraded,
    Critical,
}

#[derive(Debug, Clone)]
pub struct Fault {
    pub code: u16,
    pub severity: Severity,
    pub first_seen_ms: u64,
    pub last_seen_ms: u64,
    pub occurrences: u64,
    pub acknowledged: bool,
}

#[derive(Debug, Default)]
pub struct FaultRegistry {
    active: BTreeMap<u16, Fault>,
}

impl FaultRegistry {
    pub fn report(&mut self, code: u16, severity: Severity, now_ms: u64) {
        self.active
            .entry(code)
            .and_modify(|fault| {
                fault.last_seen_ms = now_ms.max(fault.last_seen_ms);
                fault.occurrences = fault.occurrences.saturating_add(1);
                fault.severity = severity.max(fault.severity);
            })
            .or_insert(Fault {
                code,
                severity,
                first_seen_ms: now_ms,
                last_seen_ms: now_ms,
                occurrences: 1,
                acknowledged: false,
            });
    }
    pub fn acknowledge(&mut self, code: u16) -> bool {
        if let Some(fault) = self.active.get_mut(&code) {
            fault.acknowledged = true;
            true
        } else {
            false
        }
    }
    pub fn clear(&mut self, code: u16) -> Option<Fault> {
        self.active.remove(&code)
    }
    pub fn safe_mode_required(&self) -> bool {
        self.active
            .values()
            .any(|fault| fault.severity == Severity::Critical)
    }
    pub fn active(&self) -> impl Iterator<Item = &Fault> {
        self.active.values()
    }
}

pub fn clearance_permitted(fault: &Fault, now_ms: u64, quiet_period_ms: u64) -> bool {
    fault.acknowledged
        && now_ms >= fault.last_seen_ms
        && now_ms - fault.last_seen_ms >= quiet_period_ms
}

impl FaultRegistry {
    pub fn clear_if_quiet(
        &mut self,
        code: u16,
        now_ms: u64,
        quiet_period_ms: u64,
    ) -> Option<Fault> {
        let fault = self.active.get(&code)?;
        if !clearance_permitted(fault, now_ms, quiet_period_ms) {
            return None;
        };
        self.active.remove(&code)
    }
}
