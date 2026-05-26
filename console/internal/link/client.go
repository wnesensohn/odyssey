package link

import (
	"context"
	"errors"
	"io"
	"net"
	"odyssey.example.test/console/internal/protocol"
	"sync"
	"time"
)

type Client struct {
	mu         sync.Mutex
	connection net.Conn
	sequence   uint32
	timeout    time.Duration
}

func Connect(ctx context.Context, address string, timeout time.Duration) (*Client, error) {
	if timeout <= 0 || timeout > time.Minute {
		return nil, errors.New("invalid ground-link timeout")
	}
	connection, err := (&net.Dialer{Timeout: timeout}).DialContext(ctx, "tcp", address)
	if err != nil {
		return nil, err
	}
	return &Client{connection: connection, timeout: timeout}, nil
}

func (c *Client) Exchange(ctx context.Context, kind byte, payload []byte) (protocol.Frame, error) {
	c.mu.Lock()
	defer c.mu.Unlock()
	deadline := time.Now().Add(c.timeout)
	if limit, ok := ctx.Deadline(); ok && limit.Before(deadline) {
		deadline = limit
	}
	if err := ctx.Err(); err != nil {
		return protocol.Frame{}, err
	}
	if err := c.connection.SetDeadline(deadline); err != nil {
		return protocol.Frame{}, err
	}
	expected := c.sequence
	data, err := protocol.Encode(protocol.Frame{Kind: kind, Sequence: expected, Payload: payload})
	if err != nil {
		return protocol.Frame{}, err
	}
	for len(data) > 0 {
		written, err := c.connection.Write(data)
		if err != nil {
			return protocol.Frame{}, err
		}
		if written == 0 {
			return protocol.Frame{}, io.ErrShortWrite
		}
		data = data[written:]
	}
	response, err := protocol.Read(c.connection)
	if err != nil {
		return protocol.Frame{}, err
	}
	if response.Sequence != expected {
		return protocol.Frame{}, errors.New("ground-link sequence mismatch")
	}
	c.sequence++
	return response, nil
}
func (c *Client) Close() error { return c.connection.Close() }
