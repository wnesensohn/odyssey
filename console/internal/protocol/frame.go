package protocol

import (
	"encoding/binary"
	"errors"
	"io"
)

const Version byte = 4
const MaxPayload = 1024

type Frame struct {
	Kind     byte
	Sequence uint32
	Payload  []byte
}

func CRC16(data []byte) uint16 {
	crc := uint16(0xffff)
	for _, b := range data {
		crc ^= uint16(b) << 8
		for bit := 0; bit < 8; bit++ {
			if crc&0x8000 != 0 {
				crc = crc<<1 ^ 0x1021
			} else {
				crc <<= 1
			}
		}
	}
	return crc
}

func Encode(frame Frame) ([]byte, error) {
	if !ValidKind(frame.Kind) {
		return nil, errors.New("unsupported frame kind")
	}
	if len(frame.Payload) > MaxPayload {
		return nil, errors.New("payload exceeds frame limit")
	}
	data := make([]byte, 12+len(frame.Payload))
	copy(data, "OD")
	data[2], data[3] = Version, frame.Kind
	binary.BigEndian.PutUint32(data[4:8], frame.Sequence)
	binary.BigEndian.PutUint16(data[8:10], uint16(len(frame.Payload)))
	copy(data[10:], frame.Payload)
	binary.BigEndian.PutUint16(data[len(data)-2:], CRC16(data[:len(data)-2]))
	return data, nil
}

func Decode(data []byte) (Frame, error) {
	if len(data) < 12 || !ValidKind(data[3]) || string(data[:2]) != "OD" || data[2] != Version {
		return Frame{}, errors.New("invalid frame header")
	}
	length := int(binary.BigEndian.Uint16(data[8:10]))
	if length > MaxPayload || len(data) != length+12 {
		return Frame{}, errors.New("invalid frame length")
	}
	if CRC16(data[:len(data)-2]) != binary.BigEndian.Uint16(data[len(data)-2:]) {
		return Frame{}, errors.New("invalid checksum")
	}
	return Frame{Kind: data[3], Sequence: binary.BigEndian.Uint32(data[4:8]), Payload: append([]byte(nil), data[10:10+length]...)}, nil
}

func Read(reader io.Reader) (Frame, error) {
	header := make([]byte, 10)
	if _, err := io.ReadFull(reader, header); err != nil {
		return Frame{}, err
	}
	if string(header[:2]) != "OD" || header[2] != Version {
		return Frame{}, errors.New("invalid stream header")
	}
	length := int(binary.BigEndian.Uint16(header[8:10]))
	if length > MaxPayload {
		return Frame{}, errors.New("stream payload exceeds limit")
	}
	tail := make([]byte, length+2)
	if _, err := io.ReadFull(reader, tail); err != nil {
		return Frame{}, err
	}
	return Decode(append(header, tail...))
}

func ValidKind(kind byte) bool { return kind >= 1 && kind <= 3 }
