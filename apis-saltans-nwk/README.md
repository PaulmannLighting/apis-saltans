# apis-saltans-nwk

Network-layer support types for APIS Saltans.

This crate defines small `no_std` value types for carrying Zigbee NWK context
through higher-level crates:

- `Destination` describes outgoing NWK destinations, including unicast device
  endpoints, broadcast endpoints, and APS groups.
- `Source` stores the 16-bit NWK address and, when known, the IEEE address of
  the incoming frame source.
- `Metadata` stores optional per-frame metadata such as last-hop LQI, RSSI,
  binding index, and source-route overhead.
- `Envelope<T>` combines a payload with its `Source` and `Metadata`.

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
