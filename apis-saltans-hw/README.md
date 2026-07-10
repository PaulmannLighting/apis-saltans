# apis-saltans-hw

Hardware abstraction traits and data types for Zigbee network co-processor (NCP) drivers.

This crate separates coordinator logic from concrete hardware backends. A backend implements the
implementor API (`Backend`, `Initialize`, `Driver`, and `EventTranslator`) and the startup API
(`Builder`); coordinator code receives an `NcpHandle` and uses the `Ncp` trait to send commands to
the driver actor.

## Features

- `driver-use`: exposes the driver-side types needed to start and use hardware:
  `NcpHandle`, `Builder`, and `StartedHardware`.
- `driver`: includes `driver-use` and additionally exposes the implementor-facing types:
  `Backend`, `Driver`, `EventTranslator`, `Initialize`, and `bridge`.
- `coordinator`: exposes the coordinator-facing types: `Ncp` and `WeakNcpHandle`.
- No default features are enabled. Shared data and protocol types such as `Datagram`, `Metadata`,
  `Error`, `Event`, `FoundNetwork`, `Network`, `ScannedChannel`, and `NcpHandle` are exported when
  `driver-use`, `driver`, or `coordinator` is enabled.

## Main Traits

### `Backend`

`Backend` defines the hardware-specific event type, the translator input message type, and the
`EventTranslator` implementation used by the backend.

### `Builder`

`Builder` constructs a configured backend from the endpoint descriptors exposed by the coordinator
and starts it from a hardware event stream. It relies on the associated types from `Backend` when
creating the bridge and event translator futures that connect hardware-specific events to the
crate-level event model.

### `StartedHardware`

`StartedHardware` contains the `NcpHandle`, the translated event stream, and the futures that drive
the bridge and event translator tasks. Backend-specific startup state remains internal to the
`Builder` implementation.

### `Initialize`

`Initialize` starts the command side of a backend and returns an `NcpHandle`.

### `Driver`

`Driver` is the implementor-facing command API. The sealed actor runtime receives internal
`Message` values and dispatches them to the corresponding `Driver` methods.

Transmission uses one method:

```rust
transmit(destination, datagram)
```

The `Destination` describes the APS target, and the `Datagram` contains APS profile/cluster metadata
plus the serialized application payload.

### `EventTranslator`

`EventTranslator` converts hardware-specific event messages into common `Event` values such as
network state changes, device joins/leaves, route errors, and received APS data.

### `Ncp`

`Ncp` is the caller-facing proxy trait implemented for `NcpHandle`. It sends commands to the driver
actor through a Tokio MPSC channel and waits for the one-shot response associated with each command.
