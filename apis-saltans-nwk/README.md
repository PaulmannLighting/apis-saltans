# apis-saltans-nwk

Transport-neutral Zigbee NWK context types for APIS Saltans.

This crate defines small `no_std` value types that carry network-layer context
between the hardware, APS, ZDP, ZCL, and coordinator crates. It does not parse
or serialize NWK frames. Instead, it provides the shared source, destination,
metadata, and envelope types used by layers that already have a decoded payload.

## Crate Characteristics

- `#![no_std]`
- no allocation requirement in the library
- optional `serde` derives through the `serde` feature
- optional little-endian stream derives through the `le-stream` feature
- depends on `apis-saltans-core` for Zigbee address and endpoint domain types

## Public API Overview

Top-level re-exports from `apis-saltans-nwk`:

- `Destination`
- `Envelope<T>`
- `Metadata`
- `Source`

## Types

### `Destination`

`Destination` describes an outgoing NWK transmission target:

- `Device { device, endpoint }`: one allocated device short address and one APS
  endpoint.
- `Broadcast { address, endpoint }`: one Zigbee broadcast receiver set and one
  APS endpoint selector.
- `Group { group, endpoint }`: one APS group identifier with the endpoint used
  by the sender when constructing the APS payload.

The enum stores core address wrappers (`Device`, `Broadcast`, `GroupId`, and
`Endpoint`) rather than raw integers, so callers validate address classes before
building outbound frames.

### `Source`

`Source` stores the 16-bit NWK source address and, when the backend knows it,
the source IEEE address. The IEEE address is optional because incoming frames do
not always carry enough information to resolve it.

`Source` implements `Display`, `LowerHex`, and `UpperHex` by formatting the
short address followed by the IEEE address or `N/A`.

### `Metadata`

`Metadata` stores optional data reported by the backend for an incoming frame:

- last-hop LQI
- last-hop RSSI
- binding table index
- source-route overhead

Every field is optional because hardware backends and frame paths expose
different subsets of this data.

### `Envelope<T>`

`Envelope<T>` couples an arbitrary payload with its `Source` and `Metadata`.
Higher layers use it to retain NWK context without making this crate depend on
APS, ZDP, ZCL, coordinator, or hardware frame types.

## Features

- `serde`: derives `Serialize` and `Deserialize` for the public value types.
- `le-stream`: derives little-endian stream serialization for the public value
  types.

## Example

```rust
use apis_saltans_core::IeeeAddress;
use apis_saltans_nwk::{Envelope, Metadata, Source};

let source = Source::new(0x1234, Some(IeeeAddress::new(0, 1, 2, 3, 4, 5, 6, 7)));
let metadata = Metadata::new(Some(255), Some(-42), None, Some(0));
let envelope = Envelope::new(source, metadata, [0x01, 0x02]);

assert_eq!(envelope.source().node_id(), 0x1234);
assert_eq!(envelope.metadata().last_hop_lqi(), Some(255));
assert_eq!(envelope.payload(), &[0x01, 0x02]);
```

## Related Crates In This Workspace

- `apis-saltans-core`: core Zigbee identifiers and protocol value types.
- `apis-saltans-aps`: APS frame model that consumes NWK source context.
- `apis-saltans-zdp`: Zigbee Device Profile payload model.
- `apis-saltans-zcl`: Zigbee Cluster Library payload model.
- `apis-saltans-coordinator`: higher-level coordinator API.
- `apis-saltans-hw`: hardware abstraction layer that produces NWK envelopes.
