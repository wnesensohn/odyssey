package protocol

import "testing"

func TestWideSequence1(t *testing.T) {
	b, _ := Encode(Frame{Kind: 3, Sequence: 4294967295})
	if len(b) != 12 {
		t.Fatal("incorrect header width")
	}
}
