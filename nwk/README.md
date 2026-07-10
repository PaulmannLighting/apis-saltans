# apis-saltans-nwk

Transport-neutral Zigbee NWK receive context types for APIS Saltans.

This crate defines small value types that carry network-layer context between
the hardware, APS, ZDP, ZCL, and coordinator crates. It does not parse or
serialize NWK frames. Instead, it provides the shared source, metadata, and
envelope types used by layers that already have a decoded payload.

## Crate Characteristics

- no allocation requirement in the library
- optional `serde` derives through the `serde` feature
- depends on `apis-saltans-core` for the Zigbee IEEE address domain type

## Public API Overview

Top-level public types from `apis-saltans-nwk`:

- `Envelope<T>`
- `Metadata`
- `Source`

## Types

### `Source`

`Source` stores the 16-bit NWK source address and, when the backend knows it,
the source IEEE address. The IEEE address is optional because incoming frames do
not always carry enough information to resolve it.

`Source` implements `Display`, `LowerHex`, and `UpperHex` by formatting the
short address followed by the IEEE address or `N/A`.

Useful methods:
- `new(node_id, ieee_address)`: constructs source context from raw backend data.
- `node_id()`: returns the 16-bit NWK short address.
- `ieee_address()`: returns the optional IEEE address.
- `into_parts()`: splits the source into both stored fields.

### `Metadata`

`Metadata` stores optional data reported by the backend for an incoming frame:

- last-hop LQI
- last-hop RSSI
- binding table index
- source-route overhead

Every field is optional because hardware backends and frame paths expose
different subsets of this data.

Useful methods:
- `new(...)`: constructs metadata from backend-reported optional values.
- `last_hop_lqi()`: returns the link quality indicator, if reported.
- `last_hop_rssi()`: returns the received signal strength, if reported.
- `binding_index()`: returns the backend binding table index, if reported.
- `source_route_overhead()`: returns the source-route overhead, if reported.

### `Envelope<T>`

`Envelope<T>` couples an arbitrary payload with its `Source` and `Metadata`.
Higher layers use it to retain NWK context without making this crate depend on
APS, ZDP, ZCL, coordinator, or hardware frame types.

Useful methods:
- `new(source, metadata, payload)`: constructs an envelope.
- `source()`: returns the receive-side source context.
- `metadata()`: returns the receive-side frame metadata.
- `payload()`: borrows the enclosed payload.
- `into_parts()`: consumes the envelope and returns all stored fields.

## Features

- `serde`: derives `Serialize` and `Deserialize` for the public value types and
  enables `serde` support in `apis-saltans-core`.

## Example

```rust
use zb_core::IeeeAddress;
use zb_nwk::{Envelope, Metadata, Source};

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
