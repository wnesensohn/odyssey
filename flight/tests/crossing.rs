use odyssey_flight::guidance::navigation::next_ascending_node_s;
#[test]
fn opposite_orbital_phase_has_half_period_remaining() {
    assert_eq!(
        next_ascending_node_s(6000.0, std::f64::consts::PI).unwrap(),
        3000.0
    );
}
