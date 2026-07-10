# apis-saltans-zdp Architecture

This crate models Zigbee Device Profile (ZDP) frames and service commands. ZDP commands are grouped
by service domain, then collected into a crate-wide `Command` enum that can parse and serialize by
cluster ID.

The implementation is macro-driven. Each service payload is declared once with `zdp_command!`, each
service group enum is declared with `zdp_command_group!`, and the crate-wide enum is declared with
`zdp_command_enum!`.

## Module Layout

- `src/lib.rs` is the crate facade. It re-exports frame types, service payloads, service group enums,
  the unified `Command` enum, descriptor types, and status types.
- `src/frame.rs` defines `Frame<T>`, a sequence-numbered ZDP payload wrapper, plus conversion from APS
  data frames.
- `src/frame/parse_frame_error.rs` defines errors for APS-to-ZDP frame conversion.
- `src/services.rs` re-exports all service commands, defines the `Service` trait, and declares the
  crate-wide `Command` enum.
- `src/services/` contains one module per ZDP service group:
  - `device_and_service_discovery`
  - `bind_management`
  - `network_management`
  - `security`
- `src/services/<group>.rs` defines the group enum with `zdp_command_group!` and re-exports its
  payload modules.
- `src/services/<group>/<command>.rs` defines one service request, response, or notification payload.
- `src/simple_descriptor.rs` and `src/simple_descriptor/` model the Simple Descriptor payload,
  including raw endpoint/profile IDs, application flags, and input/output cluster lists.
- `src/status.rs` and `src/status/` model ZDP status values and display helpers.
- `src/macros.rs` contains the crate-local declarative macros used by the service modules.

## Runtime Command Flow

Incoming raw APS payloads are carried as `apis_saltans_aps::Data<bytes::Bytes>`.
`TryFrom<Data<Bytes>> for Frame<Command>` validates the APS endpoints, extracts
the APS cluster ID, and then parses the payload through
`Frame<Command>::parse_with_cluster_id(cluster_id, bytes)`:

1. `Frame` consumes the first byte as the ZDP transaction sequence number.
2. The crate-private `Command::parse_with_cluster_id` selects the service group by consulting each
   group's generated cluster-ID set.
3. The selected group enum parses the payload by matching the supplied cluster ID against the
   `Cluster` implementation of its contained command types.
4. The parsed group enum is wrapped in the crate-wide `Command` enum.

Serialization flows in the reverse direction:

1. `Frame<Command>` serializes the transaction sequence number.
2. `Command` delegates `ToLeStream` to the contained service group enum.
3. The service group enum delegates `ToLeStream` to the contained command payload.

Every command payload has a static cluster ID through `apis_saltans_core::Cluster`, and every group
and top-level command enum can return the contained command's cluster ID.

## Service Organization

Each ZDP command belongs to exactly one service group enum:

- `DeviceAndServiceDiscovery`
- `BindManagement`
- `NetworkManagement`
- `Security`

The crate-wide `Command` enum contains one variant per group. This keeps parsing and serialization
layered:

- Payload structs know their fields and cluster ID.
- Group enums know which payloads belong together.
- The top-level enum knows which groups exist.

Status-dependent responses often use helper payload types. For example, address and management
responses carry a typed success payload for successful status codes and preserve the raw status code
when conversion to the crate's `Status` enum fails.

## Simple Descriptor Model

`SimpleDescriptor` represents the ZDP descriptor for one application endpoint. The wire payload
contains an endpoint ID, profile ID, device ID, application flags, input clusters, and output
clusters.

The descriptor keeps endpoint and profile identifiers in their raw integer form. This preserves
descriptor bytes that contain a reserved endpoint value or a profile unknown to `apis-saltans-core`.
Callers can use `endpoint()` and `profile()` when they need validated `Endpoint` and `Profile`
values; those methods return the rejected raw value through their error types.

`AppFlags` owns the descriptor flags byte. The application version is stored in the high nibble and
is exposed through `SimpleDescriptor::version()`.

## Macro Conventions

The macros in `src/macros.rs` are crate-local and re-exported from `lib.rs` with `pub(crate) use`.
Call sites use `crate::zdp_command!`, `crate::zdp_command_group!`, and `crate::zdp_command_enum!`.

Macro call sites should describe protocol shape. Hand-written code should be limited to constructors
that enforce invariants, custom stream implementations, custom display output, and conversions that
cannot be expressed as simple field-by-field derivations.

### `zdp_command!`

`zdp_command!` generates one ZDP service command payload struct.

Typical shape:

```rust
crate::zdp_command! {
    /// Active Endpoint Request
    derive { Copy }
    ActiveEpReq => Active_EP_req;
    cluster_id: 0x0005;
    group: DeviceAndServiceDiscovery;
    response: ActiveEpRsp;
    fields {
        nwk_addr_of_interest: u16,
    }
    getters {
        /// Returns the network address of interest.
        #[must_use]
        pub const fn nwk_addr_of_interest(&self) -> u16 {
            self.nwk_addr_of_interest
        }
    }
}
```

Generated items:

- The command struct with the declared fields.
- Derives `Clone`, `Debug`, `Eq`, `PartialEq`, and `Hash`, plus any extra derives from `derive { ... }`.
- `le_stream::FromLeStream` and `le_stream::ToLeStream` derives by default.
- Associated constants:
  - `ID: u16`
  - `NAME: &'static str`
- A default `const fn new(...) -> Self` constructor unless a `constructor { ... }` block is supplied.
- Inherent methods from the `getters { ... }` block.
- `apis_saltans_core::Cluster` for static cluster ID access.
- `Service` for static service-name access.
- `From<Box<Self>> for Self`, used by enum unboxing conversions.
- `From<Payload> for crate::Command`.
- `TryFrom<crate::Command> for Payload`.
- `ExpectResponse<crate::services::Command>` when `response: Type;` is supplied.
- `Display`, either from a custom `display { ... }` block or from a generated debug-style field list.

Optional sections:

- `derive { ... }`: extra derives such as `Copy`.
- `constructor { ... }`: custom constructor implementation. Use this when the raw fields have
  invariants or when the public constructor should accept stronger domain types.
- `getters { ... }`: inherent getter methods.
- `display { ... }`: custom `fmt` body for `std::fmt::Display`.
- `le_stream { from { ... } }`: custom `FromLeStream` implementation while deriving `ToLeStream`.
- `le_stream { to { ... } }`: custom `ToLeStream` implementation while deriving `FromLeStream`.
- `le_stream { from { ... } to { ... } }`: custom implementations for both stream traits.
- `from { ... }`: additional `From` impls emitted next to the generated type.
- `try_from { ... }`: additional `TryFrom` impls emitted next to the generated type.

Custom stream blocks should be used for payloads whose list lengths, status-dependent fields, or
wire encodings cannot be represented by a direct derive.

### `zdp_command_group!`

`zdp_command_group!` generates one service group enum.

Typical shape:

```rust
crate::zdp_command_group! {
    /// Bind management commands.
    BindManagement {
        BindReq,
        BindRsp,
        UnbindReq,
        UnbindRsp,
    }
}
```

Generated items:

- `pub enum GroupName` with one variant per command. Each variant stores `Box<CommandPayload>`.
- Derives `Clone`, `Debug`, `Eq`, `PartialEq`, and `Hash`.
- `cluster_id(&self) -> u16`, delegated to the contained command payload.
- `profile(&self) -> Profile`, delegated to the contained command payload.
- `pub(crate) cluster_ids() -> &'static [u16]`, used by the top-level parser to select the group
  without cloning the input byte iterator.
- `pub(crate) parse_with_cluster_id(cluster_id, bytes) -> Result<Option<Self>, u16>`, which attempts
  to parse the payload for known cluster IDs in the group.
- `Display`, delegated to the contained payload.
- `From<Payload> for GroupName` for each payload.
- `TryFrom<GroupName> for Payload` for each payload.
- `ToLeStream`, by generating a private iterator enum that dispatches to the payload iterator.

The group enum owns the relationship between cluster IDs and payload types inside one service domain.

### `zdp_command_enum!`

`zdp_command_enum!` generates the crate-wide `Command` enum from service group enums.

Current shape:

```rust
crate::zdp_command_enum! {
    /// Available ZDP commands.
    Command {
        DeviceAndServiceDiscovery,
        BindManagement,
        NetworkManagement,
        Security,
    }
}
```

Generated items:

- `pub enum Command` with one variant per service group.
- Derives `Clone`, `Debug`, `Eq`, `PartialEq`, and `Hash`.
- `pub(crate) parse_with_cluster_id(cluster_id, bytes) -> Result<Option<Self>, u16>`.
- `cluster_id(&self) -> u16`, delegated to the contained group enum.
- `profile(&self) -> Profile`, delegated to the contained group enum.
- `Display`, delegated to the contained group enum.
- `From<Group> for Command` for each group.
- `TryFrom<Command> for Group` for each group.
- `ToLeStream`, by generating a private iterator enum that dispatches to the group iterator.

The parser first chooses a group by checking each group's generated `cluster_ids()` list, then delegates
payload parsing to that group. If no group recognizes the cluster ID, parsing returns `Err(cluster_id)`.

## Adding a ZDP Service Command

1. Add a new command module under the appropriate `src/services/<group>/` directory.
2. Define the payload with `crate::zdp_command!`.
3. Use a custom constructor if the command has status-dependent or field-dependent invariants.
4. Use custom `le_stream` blocks if the wire representation cannot be derived.
5. Re-export the payload from the group module.
6. Add the payload type to the group's `zdp_command_group!` invocation.
7. If it belongs to a new service group, add a new group module and list the group in
   `zdp_command_enum!` in `src/services.rs`.

## Implementation Boundaries

- Payload structs are responsible for command-specific fields and invariants.
- Group enums are responsible for service-domain membership and cluster-ID dispatch within a group.
- The top-level `Command` enum is responsible for cross-group dispatch.
- Status conversion should preserve raw status bytes when conversion into `Status` is not possible.
- Length-prefixed lists should use existing sized-vector helpers when the length byte directly
  precedes the list; otherwise, use custom stream implementations.
