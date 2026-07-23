package main

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"odyssey.example.test/console/internal/protocol"
	"os"
)

func main() {
	if len(os.Args) != 2 {
		fmt.Fprintln(os.Stderr, "usage: frame-inspect HEX_FRAME")
		os.Exit(2)
	}
	data, err := hex.DecodeString(os.Args[1])
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	frame, err := protocol.Decode(data)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	if err = json.NewEncoder(os.Stdout).Encode(frame); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
