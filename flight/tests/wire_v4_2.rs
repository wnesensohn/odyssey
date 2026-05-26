#[test]
fn wire_compatibility() {
    let f = odyssey_flight::protocol::Frame {
        kind: 1,
        sequence: 0,
        payload: vec![],
    };
    let mut b = odyssey_flight::protocol::encode(&f).unwrap();
    b[2] = 3;
    assert!(odyssey_flight::protocol::decode(&b).is_err());
}
