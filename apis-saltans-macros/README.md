# apis-saltans-macros

Procedural macros used by the `apis-saltans` Zigbee crates.

This crate currently provides derive macros that generate crate-internal ZCL parser dispatch methods for enum-based command and cluster models.

## Status

This crate is an internal support crate for `apis-saltans-zcl`. Its generated code assumes the surrounding crate provides the expected ZCL types and traits.

## Provided Macros

- `ParseZclFrame`: derives `parse_zcl_frame` for an enum of ZCL command payload variants.
- `ParseZclCluster`: derives `parse_zcl_cluster` for an enum of ZCL cluster variants.

Both macros are intended for enums whose variants each contain exactly one anonymous field.

## `ParseZclFrame`

`ParseZclFrame` generates a `pub(crate)` parser that dispatches on:

- `Header::command_id()`
- `Header::control().direction()`

Each enum variant's inner type must implement:

- `crate::Command`, providing `ID` and `DIRECTION`
- `le_stream::FromLeStream`, for little-endian payload parsing

Example shape:

```rust
use apis_saltans_macros::ParseZclFrame;

#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
pub enum Command {
    On(on::Command),
    Off(off::Command),
    Toggle(toggle::Command),
}
```

The generated method has this form:

```rust
impl Command {
    pub(crate) fn parse_zcl_frame<T>(
        header: crate::Header,
        bytes: T,
    ) -> Result<Self, crate::ParseFrameError>
    where
        T: Iterator<Item = u8>;
}
```

## `ParseZclCluster`

`ParseZclCluster` generates a `pub(crate)` parser that dispatches on a ZCL cluster ID.

Each enum variant's inner type must:

- implement `apis_saltans_core::Cluster`, providing `ID`
- provide a compatible `parse_zcl_frame(header, bytes)` method

Example shape:

```rust
use apis_saltans_macros::ParseZclCluster;

#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclCluster)]
pub enum Cluster {
    Basic(basic::Command),
    OnOff(on_off::Command),
    Level(level::Command),
}
```

The generated method has this form:

```rust
impl Cluster {
    pub(crate) fn parse_zcl_cluster<T>(
        cluster_id: u16,
        header: crate::Header,
        bytes: T,
    ) -> Result<Self, crate::ParseFrameError>
    where
        T: Iterator<Item = u8>;
}
```

## Input Requirements

The derive input must be:

- an enum
- made only of single-field tuple variants
- backed by inner command or cluster types with the traits expected by the generated dispatch code

Named-field variants, unit variants, multi-field variants, and non-enum inputs are rejected by the macro.

## Error Mapping

Generated parsers return `crate::ParseFrameError`:

- `InvalidCommandId` when a ZCL command ID is not represented by the enum
- `InvalidClusterId` when a ZCL cluster ID is not represented by the enum
- `InsufficientPayload` when little-endian payload parsing fails

## Dependencies

Key dependencies:

- `syn`
- `quote`
- `proc-macro2`

## Related Workspace Crates

- `apis-saltans-zcl`: primary consumer of these derive macros
- `apis-saltans-core`: provides shared Zigbee traits and identifiers used by generated cluster dispatch
