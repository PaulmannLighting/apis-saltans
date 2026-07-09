# apis-saltans-core Architecture

`apis-saltans-core` contains shared Zigbee protocol value types. Higher-level
crates use it for addresses, endpoints, profiles, cluster identifiers, typed ZCL
values, TLVs, node descriptors, and small cross-layer traits.

The crate does not implement APS, ZCL, ZDP, coordinator behavior, or hardware
I/O. It keeps domain values and serialization helpers in one dependency that
protocol crates can share.

```mermaid
flowchart TD
    Addressing["Addressing<br/>Address, ShortId, IeeeAddress, GroupId"]
    Routing["Routing metadata<br/>Endpoint, Destination, Profile, Cluster"]
    Traits["Type metadata traits<br/>ClusterSpecific, Profiled, ExpectResponse"]
    Values["Zigbee values<br/>types::Type, units"]
    Tlv["TLVs<br/>types::tlv"]
    Node["Node model<br/>node::Node, descriptor"]
    Protocols["APS / ZCL / ZDP / coordinator / hardware"]

    Addressing --> Protocols
    Routing --> Protocols
    Traits --> Protocols
    Values --> Protocols
    Tlv --> Values
    Node --> Protocols
```

## Public Layout

| Area | Files and modules | Responsibility |
| --- | --- | --- |
| Addressing | `address.rs`, `ieee_address.rs`, `short_id.rs`, `group_id.rs` | IEEE addresses, NWK short addresses, and APS group identifiers. |
| Routing metadata | `endpoint.rs`, `destination.rs`, `profile.rs`, `cluster.rs`, `direction.rs` | Endpoint, destination, profile, cluster, and command-direction domain values. |
| Traits | `traits.rs`, `cluster.rs`, `profile.rs` | Cross-crate metadata traits such as `ExpectResponse`, `ClusterSpecific`, and `Profiled`. |
| Typed values | `types.rs`, `types/*` | Zigbee primitive, discrete, analog, composite, and tagged value representations. |
| TLVs | `types/tlv.rs`, `types/tlv/*` | Local, global, and encapsulated TLV representations. |
| Nodes | `node.rs`, `node/descriptor.rs`, `node/descriptor/*` | Node descriptors and descriptor bitfields. |
| Units | `units.rs`, `units/*` | Protocol unit wrappers such as deciseconds, mireds, and units per second. |

## Addressing Model

`IeeeAddress` and `Eui64` represent 64-bit device identifiers. `ShortId`
separates the coordinator address, allocated device short addresses, and Zigbee
broadcast short-address values. `GroupId` accepts only non-zero values in the
valid APS group range.

`Address` combines an IEEE address with a short ID for code paths that track
both identifiers together.

```mermaid
flowchart LR
    ShortId["ShortId"]
    Coordinator["Coordinator"]
    Device["short_id::Device"]
    Broadcast["short_id::Broadcast"]
    Address["Address<br/>IeeeAddress + short id"]

    Coordinator --> ShortId
    Device --> ShortId
    Broadcast --> ShortId
    ShortId --> Address
```

## Routing Metadata

`Endpoint` models the ZDO data endpoint, application endpoints, and the endpoint
broadcast value. `Destination` models outbound addressing as one of:

- a device destination,
- a broadcast destination,
- a group destination.

`Profile` is the Zigbee profile identifier enum. Its `broadcast_endpoint`
method returns the endpoint value used for profile-level broadcasts. `Cluster`
is the enum of well-known cluster identifiers defined in this crate, while
`ClusterSpecific<T = u16>` lets downstream command and attribute types expose
their own cluster ID as metadata.

`Profiled` is separate from `ClusterSpecific` so a type can expose its profile
without making profile metadata part of the cluster trait.

## Typed Values And TLVs

`types::Type` is the tagged Zigbee value enum. It wraps null values, discrete
byte blocks, booleans, dates, times, analog integers, strings, identifiers, and
other scalar values used by ZCL and ZDP payloads.

TLV support is under `types::tlv`. `Tlv<L, G>` distinguishes local and global
tag ranges, while the concrete local and global enums provide typed payloads
for known tags. TLV length fields follow the Zigbee convention where the stored
length is `payload_len - 1`.

Serialization uses `le-stream` traits. Implementations return iterators for
encoding and parse from byte iterators for decoding.

## Dependency Boundaries

The core crate is intentionally below all protocol and runtime crates:

- It may define generic domain values and serialization behavior.
- It must not depend on APS, ZCL, ZDP, coordinator, or hardware crates.
- Higher-level crates attach behavior to these values through their own command,
  frame, and dispatch types.
