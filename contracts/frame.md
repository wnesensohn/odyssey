# ODYSSEY frame version 3

All integer fields are unsigned and big-endian. A frame is exactly its declared
length; trailing bytes belong to the next stream frame and are never part of a
single-frame decode operation.

| Offset | Size | Field |
| --- | --- | --- |
| 0 | 2 | ASCII magic `OD` |
| 2 | 1 | Protocol version, 3 |
| 3 | 1 | Message kind |
| 4 | 2 | Sequence number |
| 6 | 2 | Payload length, at most 1024 |
| 8 | N | Payload |
| 8+N | 2 | CRC-16/CCITT-FALSE over preceding bytes |

Message kind 1 is a command, 2 an acknowledgement and 3 telemetry. Sequence
numbers wrap at 65535. Sessions keep a bounded pending window and acknowledgements
match the outstanding sequence. Duplicate reservations cannot replace a frame.

CRC uses polynomial 0x1021, initial state 0xffff, no reflection and no final XOR.
The ASCII vector `123456789` has checksum 0x29b1. Rust, Erlang and Go must agree.
Changes to integer widths or mandatory fields require a new wire version.
