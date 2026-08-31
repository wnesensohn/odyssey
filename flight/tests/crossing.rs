use odyssey_flight::guidance::navigation::next_ascending_node_s;
#[test]
fn opposite_orbital_phase_has_half_period_remaining() {
    assert_eq!(
        next_ascending_node_s(6000.0, std::f64::consts::PI).unwrap(),
        3000.0
    );
}

#[test]
fn exact_node_has_no_wait() {
    assert_eq!(next_ascending_node_s(6000.0, 0.0).unwrap(), 0.0);
}
#[test]
fn invalid_period_does_not_schedule_a_node() {
    assert!(next_ascending_node_s(0.0, 0.0).is_err());
}
