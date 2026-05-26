#[test]
fn wire_compatibility() {
    let f = odyssey_flight::protocol::Frame {
        kind: 3,
        sequence: u32::MAX,
        payload: vec![],
    };
    let b = odyssey_flight::protocol::encode(&f).unwrap();
    assert_eq!(b.len(), 12);
}
