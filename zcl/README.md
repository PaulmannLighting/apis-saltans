# apis-saltans-zcl

Zigbee Cluster Library (ZCL) frame, command, attribute, and cluster definitions.

This crate provides typed ZCL frame handling plus cluster command, attribute, and value models for global and
cluster-specific ZCL traffic.

## Status

This crate is under active development and currently implements a substantial, but not complete, subset of ZCL. Command
runtime dispatch and attribute coverage are tracked separately: some clusters expose attributes even when they do not yet
have cluster-specific command dispatch.

## Crate Characteristics

- always `no_std`, with allocation disabled by default
- optional `alloc` preserves dynamically allocated containers
- little-endian serialization/deserialization via `le-stream`
- depends on the local `apis-saltans-core` core crate

## Features

- `alloc`: enable allocation and forward it to APS and le-stream
- `serde`: enable serialization support without requiring allocation

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
    - global `AttributeReport` enum with `AttributeReport::parse(cluster_id, attribute_id, typ)`
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
    - OTA Upgrade
- Lighting cluster commands:
    - Color Control
- IAS cluster commands:
    - IAS Zone

The repository also contains additional cluster and attribute modules that are not yet wired into the top-level runtime
`Cluster` command dispatch enum.

## Attribute Coverage

Implemented attribute modules generate typed `Id`, `Readable`, `Writable`, `Reportable`, `SendReport`, and `Scene`
enums. `Id`/`Readable`, `Writable`, and `Scene` follow the declared access flags. `Reportable` and
`SendReport` cover all declared attributes and globals, including attributes without `P`: ZCL §2.5.7.3
allows reporting as a manufacturer option. Attribute-specific restrictions still apply, and a typed
configuration does not guarantee device support. Check Discover Attributes Extended access bits and
the Configure Reporting response. This does not grant extra read or write access: ordinary `R` remains
read-only; optional writing requires an explicit exception such as `R*W` in the specification.

`SendReport` associates attribute names with
their ZCL wire types; its variants use the attribute IDs as `u16` discriminants, while its `Reportable` implementation
returns the attribute and wire type IDs. The global readable attributes `ClusterRevision` and
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
    - OTA Upgrade
- Measurement and Sensing:
    - Illuminance Measurement
    - Illuminance Level Sensing
    - Occupancy Sensing
- Lighting:
    - Ballast Configuration
    - Color Control
- IAS:
    - IAS Zone

For reports, use `zb_zcl::AttributeReport::parse(cluster_id, attribute_id, typ)` to map a raw cluster ID,
attribute ID, and ZCL `Type` into the corresponding typed reportable attribute enum.

## OTA Upgrade

The `ota_upgrade` module implements cluster `0x0019` end to end. Its runtime command enum covers
Image Notify, Query Next Image, Image Block/Page transfer, Upgrade End, and Query Specific File
request/response commands. Status-dependent response layouts are represented by `QueryResponse`
and `ImageBlockResponsePayload`, while `ImageBlock::try_new` ensures image data fits the wire
protocol's one-octet data-size field.

OTA clients construct `UpgradeComplete::new(image)` after successful validation, or
`UpgradeIncomplete::new(reason, image)` with an `UpgradeIncompleteReason`. Both encode command
`0x06`; the successful type expects `UpgradeEndResponse`, while the incomplete type expects only
a `DefaultResponse`. Incoming packets decode into
`UpgradeEndRequest::Complete` or `UpgradeEndRequest::Incomplete` based on the status byte.
Unknown statuses and truncated payloads are rejected. Checked `TryFrom<UpgradeEndRequest>`
conversions extract a concrete request and return the original enum on a variant mismatch.

```rust
use zb_zcl::ota_upgrade::{ImageId, QueryNextImageRequest};

const MANUFACTURER_CODE: u16 = 0x1234;
const IMAGE_TYPE: u16 = 0x0001;
const CURRENT_FILE_VERSION: u32 = 0x0102_0304;
const HARDWARE_VERSION: u16 = 0x0002;

let image = ImageId::new(MANUFACTURER_CODE, IMAGE_TYPE, CURRENT_FILE_VERSION);
let request = QueryNextImageRequest::new(image, Some(HARDWARE_VERSION));
assert_eq!(request.image(), image);
```

## Serialization and Parsing

- Encode any typed frame/command with `ToLeStream`
- Parse bytes with `Frame::parse(cluster_id, bytes)` into `Frame<Cluster>`
- Construct outgoing frames without a transaction sequence using `UnsequencedHeader` and
  `UnsequencedFrame<T>`
- Convert a typed command directly with `UnsequencedFrame::from_command`
- Assign the transaction sequence by consuming an unsequenced frame with
  `UnsequencedFrame::into_frame`
- Override its default-response flag with
  `UnsequencedFrame::with_disable_default_response`
- Parse failures are surfaced as `ParseFrameError`:
    - missing header
    - invalid scope/type
    - invalid cluster/command id
    - insufficient payload

Frame, attribute, and date-code parse errors implement `core::error::Error`. Variants that retain a
lower-level parsing or invalid-type error expose it as their source and support conversion with the
`?` operator.

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

## Configure Reporting Responses

`global::configure_reporting::Response` accepts the one-byte `0x00` success payload.
Use `response.is_success()` to check whether all requested attributes were configured.
`AttributeStatus` deserialization reads the status, then attempts to read direction and
attribute ID regardless of status. Absent optional fields become `None`; a partial attribute ID fails decoding.
Success records may therefore retain fields supplied by a device; byte serialization
always omits those fields on success. Failure records serialize whichever fields are present.
`AttributeStatus::success()` constructs a success record, while `AttributeStatus::new`
preserves the supplied optional direction and attribute ID for any status.
`Response::new` and Serde deserialization preserve record lists without normalization.
`is_success()` returns true when all records indicate success, including an empty list.

## Attribute Traits

The crate provides type-safe patterns for attribute access:

- `Readable`: map attribute IDs and wire `zb_core::types::Type` values into strongly typed readable attribute
  enums/structs.
- `Writable`: convert writable attribute values into global write records.
- `Reportable`: retrieve the attribute and ZCL data type IDs from generated `SendReport` enums.
- `AttributeReport`: parse reported values across all implemented cluster attribute modules.
- `SendReport`: associate a reportable attribute with its ZCL wire type and retrieve the wire type ID.

This is intended to keep attribute handling explicit and strongly typed across clusters.

## Dependencies

Key dependencies:

- `apis-saltans-core` (core protocol definitions)
- `le-stream`
- `heapless`
- `bitflags`
- `num_enum` (primitive conversions for fieldless integer-representation enums)
- `repr-discriminant`
- `strum` (round-trippable names for generated attribute ID enums)

## Related Workspace Crates

- `apis-saltans-core`: core datatypes and TLV support
- `apis-saltans-aps`: APS frame definitions
- `apis-saltans-zdp`: ZDP service and command models

## Typed response contracts

Commands declaring a response implement `apis_saltans_core::ExpectResponse<Cluster>`. The associated
`Response` type defines all accepted response forms:

- A concrete response type requires that specific result.
- `global::default_response::DefaultResponse` accepts only a Default Response. Remove All Groups,
  Add Group If Identifying, and `UpgradeIncomplete` use this type.
- `MaybeDefault<T>` accepts either `Specific(T)` or `Default(DefaultResponse)`. Default Responses
  preserve success, failure, and unknown statuses for the caller to inspect.

`ZclResponseType` matches headers by scope and command ID. `MaybeDefault<T>` accepts the headers
matched by `T` plus Default Response headers, and its `TryFrom<Cluster>` implementation converts
either variant. Custom response enums can implement the same traits for other alternatives.
The coordinator separately checks transaction identity and the embedded original command ID.
A Default Response not accepted by the response type produces a device-status error, or an
unexpected-default error when it reports success without the required specific result.

Use the `response:` declaration in `zcl_command!` to select the complete response type, for example
`response: MaybeDefault<Response>;`. No policy constant or macro policy declaration is needed.
Response acceptance is independent of the outgoing disable-default-response flag.

Attribute value types implement `ReportingType` to select their reporting parameters.
Type aliases inherit this choice; custom types explicitly choose `Analog<T>` or `Discrete<T>`
through the associated `Configuration` type.

Read Reporting Configuration response records use `Status::reported`, `Status::received`, or
`Status::failed`. `reported` accepts an attribute ID and a `ReportedConfiguration` with named
fields; the other constructors take only their relevant values. These replace the former
eight-argument `Status::new` constructor. The record's serialized representation is unchanged.


## Allocation-free storage

The default build requires neither `std` nor an allocator. Enable `alloc` to
retain the existing `Box`, `Vec`, and `Bytes` types, boxed command variants, and
infallible command/reporting conversions. The coordinator explicitly enables
this feature. There is no `std` feature.

ZCL requests APS's `alloc` only through its own `alloc` feature. Cargo feature
unification can still enable APS allocation independently; ZCL supports that
combination. The APS input conversion uses `zb_aps::Payload`. Outgoing owned ZCL
bytes use `storage::Payload`, whose storage follows ZCL's own feature.

`storage` exposes aliases for the allocating or bounded representations.
Without allocation, variable-length lists use `RawList<T, N>`, backed by
`heapless::Vec`. Construct lists with `new`, `push`, or `try_from_iter`.
Overflow returns the rejected element or a `CapacityError`; parsing oversized
or truncated lists returns `None`. Lists retain their original **unprefixed**
encoding. Existing protocol-sized heapless strings and lists remain unchanged.
Boxed command/error values are stored inline, increasing enum sizes and stack
usage. Existing scene-extension serialization/deserialization placeholders
remain unimplemented in both modes.

### Fallible bounded APIs

- `UnsequencedFrame::<storage::Payload>::try_from_command(command)` checks the
  serialized payload size. It works in both modes; the existing `from_command`
  remains available with `alloc`.
- Generated `SendReport` conversions use `TryFrom` without allocation and retain
  `From` with allocation. Use `try_into()` when sharing code across both modes.
- `ReportingConfiguration::into_configuration` returns `ConfigurationResult`:
  a record with `alloc`, or `Result<Record, CapacityError>` without it. This
  permits custom analog values to report capacity exhaustion safely.
- Reporting command `attributes()` getters remain available in both modes;
  they are `const` only with `alloc`, because heapless slice access is not const.

### Compile-time capacities

The following `const_env` settings apply only without allocation. The build
script tracks changes to these variables and `ZCL_DISABLE_DEFAULT_RESPONSE`.
All capacities must be positive; `WIRE_CAPACITY` must be at least 269.

| Environment variable | Default | Storage |
| --- | ---: | --- |
| `ZIGBEE_ZCL_RECORDS_CAPACITY` | 4 | Attribute/reporting/discovery records |
| `ZIGBEE_ZCL_ATTRIBUTE_IDS_CAPACITY` | 64 | Requested attribute IDs |
| `ZIGBEE_ZCL_COMMAND_IDS_CAPACITY` | 64 | Discovered command IDs |
| `ZIGBEE_ZCL_INDEXES_CAPACITY` | 15 | Structured selector indexes |
| `ZIGBEE_ZCL_IMAGE_DATA_CAPACITY` | 255 | OTA image block bytes |
| `ZIGBEE_ZCL_EXTENSIONS_CAPACITY` | 16 | Scene extension fields |
| `ZIGBEE_ZCL_REPORTABLE_CHANGE_CAPACITY` | 8 | Encoded analog reportable change |
| `ZIGBEE_ZCL_PAYLOAD_CAPACITY` | 1024 | Serialized outgoing command bytes |
| `ZIGBEE_ZCL_WIRE_CAPACITY` | 269 | Scratch bytes for fixed-size OTA/selector codecs |

Protocol limits still apply independently: selectors allow at most 15 indexes,
and OTA image blocks at most 255 bytes. Smaller storage limits reject input
that does not fit. The scratch buffer's minimum accommodates a maximum OTA
block response including its status and headers, so those infallible
serializers cannot overflow it for valid values.

```sh
ZIGBEE_ZCL_RECORDS_CAPACITY=8 cargo build -p apis-saltans-zcl --no-default-features
```

Chrono runs without its standard-library defaults. Its `core-error` feature
preserves date parse error sources, while bounded date formatting writes
directly to the formatter. Illuminance conversions use `libm` for logarithms,
powers, and rounding on targets without standard-library math.

CI checks `alloc`/`serde` combinations, bare-metal builds, target dependency
features, smaller capacities, and independently enabled APS allocation.
