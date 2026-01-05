use odyssey_flight::{
    protocol::{checksum, decode, encode, Frame, MAX_PAYLOAD},
    ControlError,
};

#[test]
fn reference_checksum_matches_ccitt_false() {
    assert_eq!(checksum(b"123456789"), 0x29b1);
}
#[test]
fn frames_round_trip_at_payload_boundaries() {
    for length in [0, 1, 16, MAX_PAYLOAD] {
        let frame = Frame {
            kind: 1,
            sequence: 42,
            payload: vec![0x5a; length],
        };
        assert_eq!(decode(&encode(&frame).unwrap()).unwrap(), frame);
    }
}
#[test]
fn damaged_headers_and_payloads_are_rejected() {
    let frame = Frame {
        kind: 2,
        sequence: 65535,
        payload: vec![1, 2, 3],
    };
    let bytes = encode(&frame).unwrap();
    for index in 0..bytes.len() {
        let mut corrupt = bytes.clone();
        corrupt[index] ^= 1;
        assert_eq!(decode(&corrupt), Err(ControlError::InvalidFrame));
    }
}
#[test]
fn every_truncated_frame_is_rejected() {
    let bytes = encode(&Frame {
        kind: 3,
        sequence: 0,
        payload: vec![10; 24],
    })
    .unwrap();
    for end in 0..bytes.len() {
        assert_eq!(decode(&bytes[..end]), Err(ControlError::InvalidFrame));
    }
}
