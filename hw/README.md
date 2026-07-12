# apis-saltans-hw

Hardware abstraction traits and data types for Zigbee network co-processor (NCP) drivers.

This crate separates coordinator logic from concrete hardware backends. A backend implements the
implementor API (`Backend`, `Driver`, and `EventTranslator`); coordinator code receives an
`NcpHandle` and uses the `Ncp` trait to send commands to the driver actor.

## Features

No default features are enabled. Pick the feature that matches the role of the crate that depends on
`apis-saltans-hw`.

| Feature | Intended user | Public API |
| --- | --- | --- |
| `coordinator` | Coordinator and application code that already has a running `NcpHandle`. | `Ncp`, `NcpHandle`, `WeakNcpHandle`, `Error`, `RouteError`, `Datagram`, `Metadata`, `Event`, `FoundNetwork`, `Network`, and `ScannedChannel`. |
| `driver` | Hardware backend crates. | `Backend`, `Driver`, `EventTranslator`, `bridge`, `NcpHandle`, `WeakNcpHandle`, `Error`, `RouteError`, `Datagram`, `Metadata`, `Event`, `FoundNetwork`, `Network`, `ScannedChannel`, and protocol re-export modules. |

Backend crates should enable `driver`. Coordinator crates should enable `coordinator`.

### Using the Coordinator API

Enable `coordinator` when your code receives an `NcpHandle` from startup code and needs to send
commands to the NCP actor.

```toml
[dependencies]
apis-saltans-hw = { version = "0.7", features = ["coordinator"] }
```

Import the `Ncp` trait to make the handle methods available:

```rust,no_run
use std::time::Duration;

use apis_saltans_hw::{Ncp, NcpHandle};

async fn permit_joining(ncp: &NcpHandle) -> Result<Duration, apis_saltans_hw::Error> {
    ncp.allow_joins(Duration::from_secs(60)).await
}
```

Use this feature for command-side operations such as reading the coordinator IEEE address, scanning
networks, allowing joins, resolving addresses, requesting routes, and transmitting serialized
`Datagram` values to `zb_core::Destination` targets.

### Implementing a Driver

Enable `driver` in hardware backend crates. It exposes the traits and common data types used to
implement a backend:

```toml
[dependencies]
apis-saltans-hw = { version = "0.7", features = ["driver"] }
```

Driver crates typically implement:

- `Backend` for the backend's associated hardware event, translator message, and translator type.
- `Driver` on the NCP command actor.
- `EventTranslator` to convert backend events into common `Event` values.

Backend startup is owned by the backend crate. It should initialize the concrete driver, run the
event translator, and pass the resulting `NcpHandle` plus translated `Event` receiver to coordinator
startup code.

The `driver` feature also exposes protocol crate re-export modules:

```rust
use apis_saltans_hw::{aps, core, nwk, zdp};
```

These modules re-export `zb-aps`, `zb-core`, `zb-nwk`, and `zb-zdp` respectively. They are a
convenience for driver crates: public APIs can refer to the protocol types through
`apis_saltans_hw::core::...`, `apis_saltans_hw::aps::...`, and the other re-export modules instead
of adding direct dependencies on every protocol crate.

## Main Traits

### `Backend`

`Backend` defines the hardware-specific event type, the translator input message type, and the
`EventTranslator` implementation used by the backend.

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
network state changes, device joins/leaves with `FullAddress`, route errors, and received APS data.

### `Ncp`

`Ncp` is the caller-facing proxy trait implemented for `NcpHandle`. It sends commands to the driver
actor through a Tokio MPSC channel and waits for the one-shot response associated with each command.
