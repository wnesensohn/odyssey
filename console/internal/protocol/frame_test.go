package protocol

import (
	"bytes"
	"testing"
)

func TestReferenceChecksum(t *testing.T) {
	if CRC16([]byte("123456789")) != 0x29b1 {
		t.Fatal("CCITT-FALSE vector does not match")
	}
}
func TestFramePayloadBoundaries(t *testing.T) {
	for _, length := range []int{0, 1, 16, 1024} {
		frame := Frame{Kind: 2, Sequence: 42, Payload: bytes.Repeat([]byte{0x5a}, length)}
		encoded, err := Encode(frame)
		if err != nil {
			t.Fatal(err)
		}
		decoded, err := Decode(encoded)
		if err != nil {
			t.Fatal(err)
		}
		if decoded.Kind != frame.Kind || decoded.Sequence != frame.Sequence || !bytes.Equal(decoded.Payload, frame.Payload) {
			t.Fatal("frame changed during round trip")
		}
		streamed, err := Read(bytes.NewReader(encoded))
		if err != nil || !bytes.Equal(streamed.Payload, frame.Payload) {
			t.Fatal("stream decoder failed")
		}
	}
}
func TestEveryTruncationAndBitCorruptionIsRejected(t *testing.T) {
	data, _ := Encode(Frame{Kind: 1, Sequence: 65535, Payload: []byte{1, 2, 3, 4}})
	for end := 0; end < len(data); end++ {
		if _, err := Decode(data[:end]); err == nil {
			t.Fatalf("accepted truncation at %d", end)
		}
	}
	for index := range data {
		corrupt := bytes.Clone(data)
		corrupt[index] ^= 1
		if _, err := Decode(corrupt); err == nil {
			t.Fatalf("accepted corruption at %d", index)
		}
	}
}
func TestOversizedAndTrailingFramesAreRejected(t *testing.T) {
	if _, err := Encode(Frame{Payload: make([]byte, MaxPayload+1)}); err == nil {
		t.Fatal("accepted oversized payload")
	}
	data, _ := Encode(Frame{Payload: []byte{1}})
	if _, err := Decode(append(data, 0)); err == nil {
		t.Fatal("accepted trailing bytes")
	}
}
