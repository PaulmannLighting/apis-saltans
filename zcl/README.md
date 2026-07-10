# apis-saltans-zcl

Zigbee Cluster Library (ZCL) frame, command, attribute, and cluster definitions.

This crate provides typed ZCL frame handling plus cluster command, attribute, and value models for global and
cluster-specific ZCL traffic.

## Status

This crate is under active development and currently implements a substantial, but not complete, subset of ZCL. Command
runtime dispatch and attribute coverage are tracked separately: some clusters expose attributes even when they do not yet
have cluster-specific command dispatch.

## Crate Characteristics

- `no_std` when `std` feature is disabled
- uses `alloc` (`extern crate alloc`)
- little-endian serialization/deserialization via `le-stream`
- depends on the local `apis-saltans-core` core crate

## Features

- `std`: enable standard library mode
- `serde`: enable serde support for supported data types

## Build-Time Configuration

- `ZCL_DISABLE_DEFAULT_RESPONSE=true`: set the default value of
  `Command::DISABLE_DEFAULT_RESPONSE` for command types that do not provide an
  explicit `disable_default_response: ...;` override in their `zcl_command!`
  invocation. Outgoing frames built from those commands set the ZCL frame
  control disable-default-response bit. Commands with an explicit override keep
  their configured value.

## Public API Overview

Top-level re-exports include:

- Framing:
    - `Frame<T>`, `Header`, `Control`, `Scope`, `Direction`, `ParseFrameError`
- Commands:
    - `Command` trait
    - `CommandDispatch` trait
- Clusters:
    - `Cluster` enum (runtime command container)
    - cluster modules: `general`, `global`, `ias`, `lighting`, `measurement_and_sensing`
- Attributes:
    - `Readable`, `Writable`
    - global `Reportable` enum with `Reportable::parse(cluster_id, attribute_id, typ)`
    - `ParseAttributeError`, `InvalidType`
- Common:
    - `Status`, `Options`

## Frame Model

A ZCL frame is represented as:

- `Header`
- `payload` (`T`)

For parsed runtime payloads, use `Frame<Cluster>`. Raw APS data arrives as
`zb_aps::Data<bytes::Bytes>` and can be converted into a parsed ZCL
frame; direct byte-stream parsing is available through `Frame::parse(cluster_id,
bytes)`.

`Header` includes:

- scope (`Global` or `ClusterSpecific`)
- direction (`ClientToServer` / `ServerToClient`)
- manufacturer-specific code (optional)
- sequence number
- command ID

## Cluster Coverage (Current Runtime Dispatch)

`Cluster` currently dispatches these command groups:

- Global commands
- General cluster commands:
    - Basic
    - Groups
    - Identify
    - On/Off
    - Level
    - Alarms
    - Scenes
- Lighting cluster commands:
    - Color Control
- IAS cluster commands:
    - IAS Zone

The repository also contains additional cluster and attribute modules that are not yet wired into the top-level runtime
`Cluster` command dispatch enum.

## Attribute Coverage

Implemented attribute modules generate typed `Id`, `Readable`, `Writable`, `Reportable`, and `Scene` enums according to
the access flags present for each attribute. The global readable attributes `ClusterRevision` and
`AttributeReportingStatus` are included in every cluster attribute module.

Current attribute modules include:

- General:
    - Basic
    - Power Configuration
    - Device Temperature Configuration
    - Identify
    - Groups
    - Scenes
    - On/Off
    - Level Control
    - Alarms
    - Time
- Measurement and Sensing:
    - Illuminance Measurement
    - Illuminance Level Sensing
    - Occupancy Sensing
- Lighting:
    - Ballast Configuration
    - Color Control
- IAS:
    - IAS Zone

For reports, use `zb_zcl::Reportable::parse(cluster_id, attribute_id, typ)` to map a raw cluster ID,
attribute ID, and ZCL `Type` into the corresponding typed reportable attribute enum.

## Serialization and Parsing

- Encode any typed frame/command with `ToLeStream`
- Parse bytes with `Frame::parse(cluster_id, bytes)` into `Frame<Cluster>`
- Parse failures are surfaced as `ParseFrameError`:
    - missing header
    - invalid scope/type
    - invalid cluster/command id
    - insufficient payload

## Quick Start

### Encode a Global Default Response Payload

```rust
use le_stream::ToLeStream;
use zb_zcl::clusters::global::default_response::DefaultResponse;

let response = DefaultResponse::new(0x00, 0x01);
let bytes: Vec<u8> = response.to_le_stream().collect();
assert!(!bytes.is_empty());
```

### Parse a ZCL Frame into a Runtime Cluster Command

```rust
use zb_zcl::Frame;

let bytes = vec![0x18, 0x11, 0x0B, 0x00, 0x01];
let parsed = Frame::parse(0x0006, bytes.into_iter());
assert!(parsed.is_ok());
```

## Attribute Traits

The crate provides type-safe patterns for attribute access:

- `Readable`: map attribute IDs and wire `zb_core::types::Type` values into strongly typed readable attribute
  enums/structs.
- `Writable`: convert writable attribute values into global write records.
- `Reportable`: parse reportable attributes across all implemented cluster attribute modules.

This is intended to keep attribute handling explicit and strongly typed across clusters.

## Dependencies

Key dependencies:

- `apis-saltans-core` (core protocol definitions)
- `le-stream`
- `heapless`
- `bitflags`
- `repr-discriminant`

## Related Workspace Crates

- `apis-saltans-core`: core datatypes and TLV support
- `apis-saltans-aps`: APS frame definitions
- `apis-saltans-zdp`: ZDP service and command models
