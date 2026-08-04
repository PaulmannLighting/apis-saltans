# apis-saltans-aps

APS layer frame, data-service, and fragmentation types for Zigbee.

This crate models the Zigbee APS frame structures and the Application Support
Sublayer Data Entity (APSDE) service primitives.

## Status

This crate is under active development.

## What This Crate Provides

- APS frame control primitives:
  - `Control`
  - `FrameType`
  - `DeliveryMode`
  - `TxOptions`
- APSDE service primitives:
  - `apsde::DataRequest<A>` for `APSDE-DATA.request`
  - `apsde::DataConfirm<T>` for `APSDE-DATA.confirm`
  - `apsde::DataIndication<A, T, K>` for `APSDE-DATA.indication`
  - type-safe addressing, alias, status, and security metadata
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
- `apsde`: APS data-service primitives and transmission-option bitflags

## APS Data Service

The `apsde` module models the three APS data-service primitives without
coupling them to an actor or hardware backend. Its addressing enums encode the
address and endpoint fields permitted by each primitive:

- `RequestDestination` supports binding-table, group, 16-bit NWK unicast,
  16-bit NWK broadcast, and 64-bit IEEE destinations;
- `Destination` reports the destination of a confirmation;
- `ReceivedDestination` preserves group, 16-bit NWK unicast, 16-bit NWK broadcast, and 64-bit
  IEEE indication addressing;
- `Source` models indication source addressing;
- `NetworkAddress`, `BroadcastAddress`, and `IndividualEndpoint` reject values
  outside their respective APSDE ranges.
- `NetworkDestination` pairs a network address with an individual endpoint for
  operations that require response-capable unicast addressing.

`Alias` groups the alias source address and sequence number.
`Security<K>` groups the key index and implementation-defined device-key-pair
handle used for link-key security. ASDU length is derived from byte-like
payloads instead of being stored as independent state. `DataIndication::map_context` transforms
the backend-defined timestamp and device-key-pair handle while preserving all protocol metadata
and the ASDU.

```rust
use zb_aps::apsde::{
    DataRequest, IndividualEndpoint, NetworkAddress, RequestDestination,
};
use zb_aps::TxOptions;
use zb_core::{Endpoint, Profile};

let local_endpoint =
    IndividualEndpoint::new(Endpoint::Data).expect("endpoint 0 is individual");
let destination = RequestDestination::Network {
    address: NetworkAddress::new(0x1234).expect("valid NWK address"),
    endpoint: Endpoint::Broadcast,
};
let request = DataRequest::new(
    destination,
    Profile::ZigbeeHomeAutomation.as_u16(),
    0x0006,
    local_endpoint,
    [0x01, 0x02],
)
.with_tx_options(TxOptions::ACKNOWLEDGED_TRANSMISSION);

assert_eq!(request.asdu_length(), 2);
```

The timestamp and link-key device-pair handle types are generic because their
representations are implementation-defined. Propagated NWK and security
processing statuses retain their raw 8-bit protocol values.

## Defragmentation

`Assembler` consumes
`apsde::DataIndication<Data<bytes::Bytes>, T, K>` values. It uses the APSDE
source and APS counter to identify an in-progress fragmented transaction.
`Bytes` keeps raw APS payload handling cheap when frames are passed between
queues or reassembled from multiple fragments.

Behavior:

- unfragmented frames are returned immediately;
- first fragments start a transaction;
- follow-up fragments are inserted by block number;
- completed frames are returned with their extended header removed;
- invalid frames and out-of-bounds fragments are dropped and return `None`.

```rust
use bytes::Bytes;
use zb_aps::apsde::DataIndication;
use zb_aps::{Assembler, Data};

fn handle_frame<T, K>(
    assembler: &mut Assembler,
    indication: DataIndication<Data<Bytes>, T, K>,
) -> Option<Data<Bytes>> {
    assembler.add(indication)
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

if let Some(header) = parsed {
    assert_eq!(header.cluster_id(), 0x0006);
    assert_eq!(header.cluster(), Ok(zb_core::Cluster::OnOff));
}
```

`Header::cluster_id()` always returns the raw wire value. Use `Header::cluster()` when a typed
`zb_core::Cluster` is useful; it returns the unchanged `u16` in `Err` when the ID is not one of the
clusters known by `apis-saltans-core`.

## Notes on Safety APIs

Some constructors are intentionally marked `unsafe` (for example `new_unchecked`) when invariants must be enforced by the caller (header/content consistency). Prefer safe constructors unless you are explicitly rebuilding structures from validated external state.

## Dependencies

Primary dependencies:
- `le-stream`
- `bytes`
- `bitflags`
- `num_enum` (primitive conversions for fieldless integer-representation enums)

## Related Workspace Crates

- `apis-saltans-core`: core Zigbee protocol types
- `apis-saltans-zcl`: Zigbee Cluster Library framing and commands
- `apis-saltans-zdp`: Zigbee Device Profile services
