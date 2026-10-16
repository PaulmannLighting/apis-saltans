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

Construct `DataRequest` from a named `RequestParameters` value and its payload; optional
transmission behavior remains available through `with_tx_options`, `with_alias`, and
`with_radius_counter`. Construct `IndicationMetadata` directly with named fields, then attach
its payload using `DataIndication::new`. This makes profile and cluster IDs explicit at call sites.

APS data header constructors similarly accept `DataHeaderParameters` or
`UnicastHeaderParameters` instead of positional arguments. They derive control flags from the
supplied destination, security, acknowledgement, and extended-header settings.

`Alias` groups the alias source address and sequence number.
`Security<K>` groups the key index and implementation-defined device-key-pair
handle used for link-key security. ASDU length is derived from byte-like
payloads instead of being stored as independent state. `DataIndication::map_context` transforms
the backend-defined timestamp and device-key-pair handle while preserving all protocol metadata
and the ASDU.

```rust
use zb_aps::apsde::{
    DataRequest, IndividualEndpoint, NetworkAddress, RequestDestination, RequestParameters,
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
    RequestParameters {
        destination,
        profile_id: Profile::ZigbeeHomeAutomation.as_u16(),
        cluster_id: 0x0006,
        source_endpoint: local_endpoint,
    },
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

`apis-saltans-core::NetworkError` describes portable networking failures. A backend can attach one to an
APS confirmation using `ConfirmStatus::Failure(error)`. Unclassified failures retain their native
status byte in `NetworkError::BackendStatus(u8)`; recognized failures carry portable semantics. `ConfirmStatus::network_error()` also classifies known native APS failures; it never
interprets an untyped network status byte as a vendor-specific code.

## `no_std` and storage features

APS is `no_std` and has no default features. Without `alloc`, owned payloads and the assembler
use fixed-capacity heapless storage. Enable `alloc` to use the original `bytes::Bytes`,
`alloc::vec::Vec`, and `alloc::collections::BTreeMap` representations:

```toml
[dependencies]
apis-saltans-aps = { version = "0.12", features = ["alloc"] }
```

`Payload` is an alias for `bytes::Bytes` with `alloc`. Without it, `Payload` is an owned,
heapless byte buffer with `TryFrom<&[u8]>`, `try_from_iter`, slice access, and little-endian
stream traits. Its wire encoding is raw bytes, with **no length prefix**. The generic `Data<T>`
and APSDE payload types remain usable with other representations, including borrowed data.

```rust
use apis_saltans_aps::Payload;

#[cfg(not(feature = "alloc"))]
let payload = Payload::try_from(&[1, 2, 3][..]).expect("payload fits");
#[cfg(feature = "alloc")]
let payload = Payload::from_static(&[1, 2, 3]);
assert_eq!(payload.as_ref(), &[1, 2, 3]);
```

### Compile-time capacities

Without `alloc`, `const_env` reads these environment variables when APS is compiled:

| Public constant | Environment variable | Default | Unit |
| --- | --- | ---: | --- |
| `PAYLOAD_CAPACITY` | `ZIGBEE_APS_PAYLOAD_CAPACITY` | 1024 | Owned/reassembled payload bytes |
| `FRAGMENT_CAPACITY` | `ZIGBEE_APS_FRAGMENT_CAPACITY` | 128 | Bytes per buffered fragment |
| `FRAGMENTS_CAPACITY` | `ZIGBEE_APS_FRAGMENTS_CAPACITY` | 16 | Fragment slots per transaction |
| `TRANSACTIONS_CAPACITY` | `ZIGBEE_APS_TRANSACTIONS_CAPACITY` | 4 | Concurrent transactions |

These are implementation resource limits, not additional Zigbee wire-format limits. All must be
positive; fragment capacity must not exceed payload capacity, and fragment slots must be at most
255. Invalid configurations fail compilation. They are ignored with `alloc`.

For example:

```sh
ZIGBEE_APS_PAYLOAD_CAPACITY=512 ZIGBEE_APS_FRAGMENT_CAPACITY=64 \
ZIGBEE_APS_FRAGMENTS_CAPACITY=8 ZIGBEE_APS_TRANSACTIONS_CAPACITY=2 \
cargo check -p apis-saltans-aps --target thumbv7em-none-eabi
```

The build script tracks changes to these variables so Cargo rebuilds APS when they change.
Configure them in the build environment or Cargo's `[env]` configuration; they are not runtime settings.
The assembler's inline memory scales approximately as transactions × fragment slots × fragment
capacity, plus headers, lengths, discriminants, and temporary output storage. The default fragment
byte storage alone totals 8 KiB. Cloning and splitting bounded payloads copies bytes; allocating
mode retains `Bytes` sharing and splitting behavior.

### Errors and fragmentation

`Assembler::try_add()` returns `Result<Option<Data<Payload>>, AssemblyError>`: `Ok(None)` means an
incomplete transaction, while errors distinguish invalid fragments, unexpected follow-ups, and
resource exhaustion. `CapacityError` identifies the resource, required size, and configured limit.
`Assembler::add()` remains a compatibility wrapper returning `None` for rejected input.

A full table rejects new keys without evicting other transactions. A valid first fragment can
replace the same key even when the table is full; an invalid replacement leaves the existing
transaction intact. Invalid or excessive follow-up fragments discard their matching transaction.
Duplicate blocks replace their previous bytes and are counted only once toward the total size.
Single-block and unfragmented frames do not occupy transaction slots and only need to fit the owned
payload capacity. Nothing is silently truncated on overflow.

`Data<Payload>::fragment()` preserves the existing `TryFromIntError` error type with `alloc`.
Without `alloc`, `FragmentationError` also reports excessive chunk sizes or fragment counts before
returning an iterator. Reassembly independently checks total bytes, since incoming peers need not
respect local capacity settings.

CI builds and tests APS in isolation in both modes, checks bare-metal compilation, and exercises
smaller capacities. Other workspace crates explicitly enable APS's `alloc` feature to retain their
existing APIs; building the whole workspace therefore does not test allocation-free APS.
