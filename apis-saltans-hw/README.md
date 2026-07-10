# apis-saltans-hw

Hardware abstraction traits and data types for Zigbee network co-processor (NCP) drivers.

This crate separates coordinator logic from concrete hardware backends. A backend implements the
driver API (`Builder`, `Initialize`, `Driver`, and `EventTranslator`); coordinator code receives
an `NcpHandle` and uses the `Ncp` trait to send commands to the driver actor.

## Features

- `driver`: exposes the driver-facing types: `Builder`, `Driver`, `EventTranslator`, `Initialize`,
  `PreparedHardware`, and `bridge`.
- `coordinator`: exposes the coordinator-facing types: `Ncp` and `WeakNcpHandle`.
- No default features are enabled. Shared data and protocol types such as `Datagram`, `Metadata`,
  `Error`, `Event`, `FoundNetwork`, `Network`, `ScannedChannel`, and `NcpHandle` are always
  exported.

## Main Traits

### `Builder`

`Builder` constructs a hardware backend from the endpoint descriptors exposed by the coordinator.
It also prepares the bridge and event translator tasks that connect hardware-specific event streams
to the crate-level event model.

### `Initialize`

`Initialize` starts the command side of a prepared backend and returns an `NcpHandle`.

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
