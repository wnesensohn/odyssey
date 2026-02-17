use odyssey_flight::power::power::*;

#[test]
fn nominal_behavior() {
    assert!(unsupplied_loads(100.0, &[]).unwrap().is_empty());
}

#[test]
fn boundary_behavior() {
    assert_eq!(
        unsupplied_loads(
            0.0,
            &[Load {
                id: 1,
                demand_w: 10.0,
                priority: Priority::Payload
            }]
        )
        .unwrap(),
        vec![1]
    );
}

#[test]
fn invalid_behavior() {
    assert!(unsupplied_loads(-1.0, &[]).is_err());
}
