# apis-saltans-core

Core Zigbee protocol definitions used by the workspace crates (`apis-saltans-aps`, `apis-saltans-zcl`, `apis-saltans-zdp`, coordinator, and hardware integrations).

This crate provides:
- protocol-level data types (`types::Type`, scalar wrappers, strings, dates/times)
- Type-Length-Value (TLV) structures (`types::tlv`)
- core identifiers and enums (`Endpoint`, `ShortId`, `Device`, `Profile`, `Direction`, `Cluster`)
- destination, address, group, and node representation (`Destination`, `FullAddress`, `GroupId`, `node::Node`, descriptor bitfields)
- utility traits used across protocol layers (`ExpectResponse`, `ClusterSpecific`, `Profiled`, `TypeId`)

## Status

This crate is an actively developed core layer and should be treated as work in progress.

## Crate Characteristics

- `#![no_std]`
- no runtime `alloc` dependency in the library itself
- `alloc` is currently used in tests only
- little-endian serialization/deserialization via [`le-stream`](https://crates.io/crates/le-stream)
- optional `serde` support via the `serde` feature

## Installation

Add the crate from the workspace or repository path.

```toml
[dependencies]
zb-core = { path = "../core" }
```

Enable serde support if needed:

```toml
[dependencies]
zb-core = { path = "../core", features = ["serde"] }
```

## Public API Overview

Top-level re-exports from `apis-saltans-core`:
- `ByteSizedVec`
- `Cluster`, `ClusterSpecific`
- `Destination`
- `Device`
- `Direction`
- `Endpoint`, `Application`
- `Eui64`, `IeeeAddress`
- `FullAddress`
- `GroupId`
- `Profile`, `Profiled`
- `ShortId`
- `ExpectResponse`, `TypeId`
- modules: `constants`, `destination`, `endpoint`, `node`, `short_id`, `types`, `units`

Key modules:
- `types`: Zigbee primitive/composite/discrete types and the tagged `Type` enum
- `types::tlv`: local/global TLV variants and TLV vectors
- `node`: node descriptors and capability flags
- `short_id`: device, coordinator, and broadcast NWK short-address values
- `endpoint`: data, application, and broadcast endpoint values
- `destination`: outbound device, broadcast, and group destinations
- `units`: shared unit wrappers (`Deciseconds`, `Mireds`, `UnitsPerSecond`)

Nested domain helpers:
- `destination::Device` and `destination::Broadcast`: destination payloads for `Destination`.
- `endpoint::Broadcast` and `endpoint::Reserved`: endpoint selectors used for broadcast delivery and rejected reserved IDs.
- `short_id::Device` and `short_id::Broadcast`: validated NWK short-address subranges.
- `FullAddress`: a resolved pair of IEEE address and NWK short address for one device.

## Serialization Model

Most types in this crate implement one or more of:
- `le_stream::ToLeStream` for encoding to byte iterators
- `le_stream::FromLeStream` for decoding from byte iterators
- `le_stream::FromLeStreamTagged` for tag-dispatched decoding

`ToLeStream` returns iterators rather than allocating buffers. Collect only when you need owned bytes.

## Zigbee `Type` Values

`types::Type` is the central tagged value enum for Zigbee payload typing. It covers:
- nulls (`Unknown`, `NoData`)
- discrete data blocks (`Data8`..`Data64`, `Bool`, dates/times)
- analog integers (`Uint8`..`Uint64`, `Int8`..`Int64` and non-native widths)
- composite types (`OctetString`, `String`)
- protocol identifiers (`ClusterId`, `AttributeId`, `BacnetObjectId`, `IeeeAddress`, `Key128`)

`Type` supports round-tripping by tag via `FromLeStreamTagged` and bytes via `ToLeStream`.
Its payload types implement `TypeId`, whose `ID` constant is the corresponding
Zigbee data type tag. Payloads with otherwise identical representations use distinct
transparent newtypes, such as `Uint8` and `Enum8`.

## TLV Encoding Notes

The TLV types in `types::tlv` follow the Zigbee style:
- `Tag` (1 byte)
- `Length` (1 byte)
- `Payload` (`Length + 1` bytes)

This means the stored length byte is `payload_len - 1`.

Examples:
- a 2-byte payload uses `length = 1`
- `SymmetricPassphrase` payload is 16 bytes, so its length byte is `15`

The crate models this consistently in parsing and serialization across TLV implementations.

## Working With TLVs

`types::tlv::Tlv` is generic over local/global TLV enums and dispatches by tag range:
- local tags: `0..=63`
- global tags: `64..=255`

Useful types:
- `types::tlv::Tlv`
- `types::tlv::TlvVec<T>` (length-prefixed vector with Zigbee TLV size convention)
- `types::tlv::Local`
- `types::tlv::Global`
- `types::tlv::EncapsulatedGlobal`

## Examples

### Encode a Global TLV

```rust
use le_stream::ToLeStream;
use zb_core::types::tlv::{Global, SymmetricPassphrase, Tlv};

let passphrase = SymmetricPassphrase::new([0xAA; 16]);
let tlv = Tlv::Global(Global::SymmetricPassphrase(passphrase));
let bytes = tlv.to_le_stream().collect::<heapless::Vec<u8, 18>>();

// bytes[0] = tag, bytes[1] = 15, bytes[2..] = 16-byte payload
```

### Decode a Typed Value by Tag

```rust
use le_stream::FromLeStreamTagged;
use zb_core::types::Type;

let tag = 0x21; // Uint16
let payload = [0x34, 0x12];
let value = Type::from_le_stream_tagged(tag, payload.into_iter())
    .expect("known tag")
    .expect("enough bytes");

assert!(matches!(value, Type::Uint16(_)));
```

### Convert Endpoints, Devices, Profiles, and Clusters

```rust
use zb_core::{Application, Cluster, Device, Endpoint, Profile};

let application: Application = "1".parse().expect("valid application endpoint");
let ep = Endpoint::from(application);
let broadcast: Endpoint = "0xff".parse().expect("broadcast endpoint");
let device: Device = "ColorDimmableLight".parse().expect("known device");
let profile: Profile = "ZigbeeHomeAutomation".parse().expect("known profile");
let cluster: Cluster = "0x0300".parse().expect("known cluster");

assert_eq!(device.as_u16(), 0x0102);
assert_eq!(device.to_string(), "ColorDimmableLight (0x0102)");
assert_eq!(profile.as_u16(), 0x0104);
assert_eq!(profile.to_string(), "ZigbeeHomeAutomation (0x0104)");
assert_eq!(cluster, Cluster::ColorControl);
assert_eq!(cluster.to_string(), "ColorControl (0x0300)");
assert_eq!(u8::from(ep), 1);
assert_eq!(broadcast, Endpoint::Broadcast);
```

## Error Handling

Identifier parsing and date/time conversion return small, typed errors that implement
`core::error::Error`. Their `Display` messages describe the rejected domain value or invariant, and
wrapped source errors remain available through `Error::source` where applicable.

## Traits

- `ExpectResponse<T>`: associates command/request types with response types.
- `ClusterSpecific<T = u16>`: associates a type with a Zigbee cluster ID.
- `Profiled`: associates a type with a Zigbee profile.
- `TypeId`: associates a value type with its Zigbee data type ID.

## Dependencies

Key dependencies:

- `le-stream` for little-endian wire encoding and decoding
- `num_enum` for primitive conversions on fieldless integer-representation enums
- `repr-discriminant` for discriminants on payload-carrying enums
- `strum` for round-trippable text representations of fieldless enums
- `chrono` for Zigbee date and time conversion
- `bitflags` for protocol flag and capability fields

## Related Crates In This Workspace

- `apis-saltans-aps`: Zigbee APS layer
- `apis-saltans-zcl`: Zigbee Cluster Library
- `apis-saltans-zdp`: Zigbee Device Profile
- `apis-saltans-coordinator`: actor-based coordinator API
- `apis-saltans-hw`: hardware abstraction layer

## Legal

This project is independent and not affiliated with the Zigbee Alliance.
