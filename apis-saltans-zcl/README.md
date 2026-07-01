# apis-saltans-zcl

Zigbee Cluster Library (ZCL) frame, command, attribute, and cluster definitions.

This crate provides typed ZCL frame handling plus a broad set of cluster command/data models for global and cluster-specific commands.

## Status

This crate is under active development and currently implements a substantial, but not complete, subset of ZCL.

## Crate Characteristics

- `no_std` when `std` feature is disabled
- uses `alloc` (`extern crate alloc`)
- little-endian serialization/deserialization via `le-stream`
- depends on the local `apis-saltans-core` core crate

## Features

- `std`: enable standard library mode
- `serde`: enable serde support for supported data types
- `smarthomelib`: optional integration support

## Public API Overview

Top-level re-exports include:
- Framing:
  - `Frame<T>`, `Header`, `Control`, `Scope`, `Direction`, `ParseFrameError`
- Commands:
  - `Command` trait
  - `CommandDispatch` trait
- Clusters:
  - `Cluster` enum (runtime command container)
  - cluster modules: `general`, `global`, `lighting`, `measurement_and_sensing`
- Attributes:
  - `ReadableAttribute`, `WritableAttribute`
  - `ParseAttributeError`, `InvalidType`
- Common:
  - `Status`, `Options`

## Frame Model

A ZCL frame is represented as:
- `Header`
- `payload` (`T`)

For parsed runtime payloads, use `Frame<Cluster>` and `Frame::parse(cluster_id, bytes)`.

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
- Lighting cluster commands:
  - Color Control

The repository also contains additional cluster/type modules that are not yet wired into the top-level runtime `Cluster` dispatch enum.

## Serialization and Parsing

- Encode any typed frame/command with `ToLeStream`
- Parse bytes with `Frame::parse(cluster_id, bytes)` into `Frame<Cluster>`
- Parse failures are surfaced as `ParseFrameError`:
  - missing header
  - invalid scope/type
  - invalid cluster/command id
  - insufficient payload

## Quick Start

### Encode a Global Default Response Frame

```rust
use le_stream::ToLeStream;
use apis_saltans_zcl::clusters::global::{Command as GlobalCommand, default_response::DefaultResponse};
use apis_saltans_zcl::{Cluster, Direction, Frame, Header, Scope};

let payload = Cluster::Global(GlobalCommand::DefaultResponse(DefaultResponse::new(0x00, 0x01)));
let header = Header::new(
    Scope::Global,
    Direction::ClientToServer,
    true,  // disable default response
    None,  // no manufacturer code
    0x11,  // sequence
    0x0B,  // Default Response command id
);

let frame = unsafe { Frame::new_unchecked(header, payload) };
let bytes: Vec<u8> = frame.to_le_stream().collect();
assert!(!bytes.is_empty());
```

### Parse a ZCL Frame into a Runtime Cluster Command

```rust
use apis_saltans_zcl::Frame;

let bytes = vec![0x18, 0x11, 0x0B, 0x00, 0x01];
let parsed = Frame::parse(0x0006, bytes.into_iter());
assert!(parsed.is_ok());
```

## Attribute Traits

The crate provides type-safe patterns for attribute access:
- `ReadableAttribute`: map attribute IDs and wire `apis_saltans_core::types::Type` values into strongly typed attribute enums/structs
- `WritableAttribute`: convert attribute write structures into global write records

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
