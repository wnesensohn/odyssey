use odyssey_flight::avionics::fault::*;
use odyssey_flight::fault::Severity;
fn review_fault(acknowledged: bool, last_seen_ms: u64) -> Fault {
    Fault {
        code: 1,
        severity: Severity::Degraded,
        first_seen_ms: 0,
        last_seen_ms,
        occurrences: 1,
        acknowledged,
    }
}

#[test]
fn nominal_behavior() {
    assert!(clearance_permitted(&review_fault(true, 100), 200, 100));
}
