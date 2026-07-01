# apis-saltans-core

Core Zigbee protocol definitions used by the workspace crates (`apis-saltans-aps`, `apis-saltans-zcl`, `apis-saltans-zdp`, coordinator, and hardware integrations).

This crate provides:
- protocol-level data types (`types::Type`, scalar wrappers, strings, dates/times)
- Type-Length-Value (TLV) structures (`types::tlv`)
- core identifiers and enums (`Endpoint`, `Profile`, `Direction`, `ClusterId`)
- address and node representation (`Address`, `node::Node`, descriptor bitfields)
- utility traits used across protocol layers (`ExpectResponse`, decisecond conversions)

## Status

This crate is an actively developed core layer and should be treated as work in progress.

## Crate Characteristics

- `#![no_std]`
- no runtime `alloc` dependency in the library itself
- `alloc` is currently used in tests only
- little-endian serialization/deserialization via [`le-stream`](https://crates.io/crates/le-stream)
- optional `serde` support via the `serde` feature

## Installation

Add the crate from the workspace or repository path (the crate is currently configured with `publish = false`).

```toml
[dependencies]
apis_saltans_core = { path = "../apis-saltans-core" }
```

Enable serde support if needed:

```toml
[dependencies]
apis_saltans_core = { path = "../apis-saltans-core", features = ["serde"] }
```

## Public API Overview

Top-level re-exports from `apis-saltans-core`:
- `Address`
- `Cluster`, `ClusterId`, `ClusterSpecific`
- `Direction`
- `Endpoint`, `Application`, `Reserved`
- `Profile`
- `ExpectResponse`, `FromDeciSeconds`, `IntoDeciSeconds`
- modules: `constants`, `node`, `types`, `units`

Key modules:
- `types`: Zigbee primitive/composite/discrete types and the tagged `Type` enum
- `types::tlv`: local/global TLV variants and TLV vectors
- `node`: node descriptors and capability flags
- `units`: shared unit wrappers (for example `Mireds`)

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
use apis_saltans_core::types::tlv::{Global, SymmetricPassphrase, Tlv};

let passphrase = SymmetricPassphrase::new([0xAA; 16]);
let tlv = Tlv::Global(Global::SymmetricPassphrase(passphrase));
let bytes = tlv.to_le_stream().collect::<heapless::Vec<u8, 18>>();

// bytes[0] = tag, bytes[1] = 15, bytes[2..] = 16-byte payload
```

### Decode a Typed Value by Tag

```rust
use le_stream::FromLeStreamTagged;
use apis_saltans_core::types::Type;

let tag = 0x21; // Uint16
let payload = [0x34, 0x12];
let value = Type::from_le_stream_tagged(tag, payload.into_iter())
    .expect("known tag")
    .expect("enough bytes");

assert!(matches!(value, Type::Uint16(_)));
```

### Convert Endpoints and Profiles

```rust
use apis_saltans_core::{Endpoint, Profile};

let ep = Endpoint::from(1u8);
let profile = Profile::try_from(0x0104).expect("known profile");

assert_eq!(u16::from(profile), 0x0104);
assert_eq!(u8::from(ep), 1);
```

## Traits

- `ExpectResponse<T>`: associates command/request types with response types.
- `IntoDeciSeconds`: converts values (for example `core::time::Duration`) into deciseconds.
- `FromDeciSeconds`: constructs values from deciseconds.

## Related Crates In This Workspace

- `apis-saltans-aps`: Zigbee APS layer
- `apis-saltans-zcl`: Zigbee Cluster Library
- `apis-saltans-zdp`: Zigbee Device Profile
- `apis-saltans-coordinator`: actor-based coordinator API
- `apis-saltans-hw`: hardware abstraction layer

## Legal

This project is independent and not affiliated with the Zigbee Alliance.
