# apis-saltans-aps

APS layer frame definitions and utilities for Zigbee.

This crate models the Zigbee APS frame structures (data, command, and acknowledgment paths), including control fields, delivery modes, destinations, and extended header metadata.

## Status

This crate is under active development.

## What This Crate Provides

- APS frame control primitives:
  - `Control`
  - `FrameType`
  - `DeliveryMode`
- Addressing and destination modeling:
  - `Destination` (unicast, broadcast endpoint, group)
  - `Broadcast` (well-known Zigbee broadcast addresses)
- Frame structures:
  - `Data<T>` (generic APS data frame)
  - `Unicast<T>` (typed unicast variant)
  - `Command<T>` (APS command frame)
  - `Acknowledgement` (APS ACK frame)
- Defragmentation:
  - `Assembler` (stateful APS data-frame reassembly)
- Extended header support:
  - `Extended`
  - `ExtendedControl`
  - `Fragmentation`
  - `AckFmt`

Top-level re-exports are available from `apis-saltans-aps` directly.

## Crate Layout

- `frame::control`: APS frame control bitfields and decoding helpers
- `frame::data`: APS data frame headers and payload wrappers
- `frame::command`: APS command frame/header types
- `frame::acknowledgement`: APS acknowledgment frame and ack format
- `frame::extended`: extended header fields and fragmentation
- `frame::data::defragmentation`: stateful reassembly of fragmented APS data frames
- `broadcast`: Zigbee network broadcast addresses

## Defragmentation

`Assembler` consumes `zb_nwk::Envelope<Data<bytes::Bytes>>` values.
It uses the NWK source and APS counter to identify an in-progress fragmented
transaction. `Bytes` keeps raw APS payload handling cheap when frames are passed
between queues or reassembled from multiple fragments.

Behavior:

- unfragmented frames are returned immediately;
- first fragments start a transaction;
- follow-up fragments are inserted by block number;
- completed frames are returned with their extended header removed;
- invalid frames and out-of-bounds fragments are dropped and return `None`.

```rust
use zb_aps::{Assembler, Data};
use bytes::Bytes;
use zb_nwk::Envelope;

fn handle_frame(
    assembler: &mut Assembler,
    envelope: Envelope<Data<Bytes>>,
) -> Option<Data<Bytes>> {
    assembler.add(envelope)
}
```

## Serialization

This crate uses `le-stream` for little-endian byte encoding/decoding.

Patterns used in the API:
- `ToLeStream` for serialization to iterators
- `FromLeStream` for parsing selected frame/header types

Most frame builders produce strongly typed structures first, then serialize via `to_le_stream()`.

## Quick Start

### Build and Serialize a Unicast APS Data Frame

```rust
use zb_aps::Unicast;
use le_stream::ToLeStream;

let frame = Unicast::new(
    false,  // security
    true,   // ack_request
    1,      // dst endpoint
    0x0006, // cluster id
    0x0104, // profile id
    1,      // src endpoint
    0x2A,   // APS counter
    None,   // no extended header
    [0x01, 0x02, 0x03],
);

let bytes: Vec<u8> = frame.to_le_stream().collect();
assert!(!bytes.is_empty());
```

### Parse an APS Data Header

```rust
use zb_aps::data::Header;
use le_stream::FromLeStream;

let raw = [
    0b0000_0000, // control (example)
    0x01,        // destination endpoint (delivery mode dependent)
    0x06, 0x00,  // cluster id
    0x04, 0x01,  // profile id
    0x01,        // source endpoint
    0x2A,        // APS counter
];

let parsed = Header::from_le_stream(raw.into_iter());
```

## Notes on Safety APIs

Some constructors are intentionally marked `unsafe` (for example `new_unchecked`) when invariants must be enforced by the caller (header/content consistency). Prefer safe constructors unless you are explicitly rebuilding structures from validated external state.

## Dependencies

Primary dependencies:
- `le-stream`
- `bytes`
- `bitflags`
- `num-traits` / `num-derive`

## Related Workspace Crates

- `apis-saltans-core`: core Zigbee protocol types
- `apis-saltans-zcl`: Zigbee Cluster Library framing and commands
- `apis-saltans-zdp`: Zigbee Device Profile services
