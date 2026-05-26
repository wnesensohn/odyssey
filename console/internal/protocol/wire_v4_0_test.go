package protocol

import "testing"

func TestWideSequence0(t *testing.T) {
	f := Frame{Kind: 1, Sequence: 70000, Payload: []byte{1, 2}}
	b, _ := Encode(f)
	decoded, err := Decode(b)
	if err != nil || decoded.Sequence != 70000 {
		t.Fatal(decoded, err)
	}
}
