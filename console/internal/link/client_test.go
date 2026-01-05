package link

import (
	"context"
	"net"
	"odyssey.example.test/console/internal/protocol"
	"testing"
	"time"
)

func TestExchangeMatchesResponseSequence(t *testing.T) {
	a, b := net.Pipe()
	defer a.Close()
	defer b.Close()
	client := &Client{connection: a, timeout: time.Second}
	go func() {
		frame, err := protocol.Read(b)
		if err != nil {
			return
		}
		frame.Kind = 2
		data, _ := protocol.Encode(frame)
		_, _ = b.Write(data)
	}()
	response, err := client.Exchange(context.Background(), 1, []byte("nominal"))
	if err != nil {
		t.Fatal(err)
	}
	if response.Kind != 2 || string(response.Payload) != "nominal" {
		t.Fatal("response changed payload")
	}
}
func TestExchangeRejectsUnrelatedAcknowledgement(t *testing.T) {
	a, b := net.Pipe()
	defer a.Close()
	defer b.Close()
	client := &Client{connection: a, timeout: time.Second}
	go func() {
		frame, err := protocol.Read(b)
		if err != nil {
			return
		}
		frame.Sequence++
		data, _ := protocol.Encode(frame)
		_, _ = b.Write(data)
	}()
	if _, err := client.Exchange(context.Background(), 1, nil); err == nil {
		t.Fatal("accepted mismatched sequence")
	}
}
func TestCancelledExchangeDoesNotWrite(t *testing.T) {
	a, b := net.Pipe()
	defer a.Close()
	defer b.Close()
	client := &Client{connection: a, timeout: time.Second}
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	if _, err := client.Exchange(ctx, 1, nil); err == nil {
		t.Fatal("accepted cancelled request")
	}
}
