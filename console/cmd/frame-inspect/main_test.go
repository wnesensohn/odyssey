package main

import (
	"encoding/hex"
	"odyssey.example.test/console/internal/protocol"
	"os/exec"
	"strings"
	"testing"
)

func TestInspectExecutableReadsWideSequence(t *testing.T) {
	frame, _ := protocol.Encode(protocol.Frame{Kind: 1, Sequence: 70000, Payload: []byte{1, 2}})
	output, err := exec.Command("go", "run", ".", hex.EncodeToString(frame)).CombinedOutput()
	if err != nil || !strings.Contains(string(output), `"Sequence":70000`) {
		t.Fatal(string(output), err)
	}
}
func TestInspectExecutableRejectsCorruptFrame(t *testing.T) {
	if exec.Command("go", "run", ".", "0001").Run() == nil {
		t.Fatal("invalid frame accepted")
	}
}
