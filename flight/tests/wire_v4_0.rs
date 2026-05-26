#[test]
fn wire_compatibility() {
    let f = odyssey_flight::protocol::Frame {
        kind: 1,
        sequence: 70000,
        payload: vec![1, 2],
    };
    assert_eq!(
        odyssey_flight::protocol::decode(&odyssey_flight::protocol::encode(&f).unwrap()).unwrap(),
        f
    );
}
