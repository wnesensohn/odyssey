package protocol

import "testing"

func TestWideSequence2(t *testing.T) {
	b, _ := Encode(Frame{Kind: 1})
	b[2] = 3
	if _, err := Decode(b); err == nil {
		t.Fatal("old peer accepted")
	}
}
