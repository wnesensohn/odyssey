use odyssey_flight::power::power::*;

#[test]
fn nominal_behavior() {
    assert!(restore_order(&[], 100.0, 20.0).unwrap().is_empty());
}

#[test]
fn boundary_behavior() {
    assert_eq!(
        restore_order(
            &[Load {
                id: 1,
                demand_w: 90.0,
                priority: Priority::Payload
            }],
            100.0,
            20.0
        )
        .unwrap(),
        Vec::<u16>::new()
    );
}

#[test]
fn invalid_behavior() {
    assert!(restore_order(&[], 100.0, 200.0).is_err());
}
