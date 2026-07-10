# apis-saltans-hw

Hardware abstraction traits and data types for Zigbee network co-processor (NCP) drivers.

This crate separates coordinator logic from concrete hardware backends. A backend implements the
implementor API (`Backend`, `Driver`, and `EventTranslator`) and the startup API
(`Builder`); coordinator code receives an `NcpHandle` and uses the `Ncp` trait to send commands to
the driver actor.

## Features

No default features are enabled. Pick the feature that matches the role of the crate that depends on
`apis-saltans-hw`.

| Feature | Intended user | Public API |
| --- | --- | --- |
| `coordinator` | Coordinator and application code that already has a running `NcpHandle`. | `Ncp`, `NcpHandle`, `WeakNcpHandle`, `Error`, `RouteError`, `Datagram`, `Metadata`, `Event`, `FoundNetwork`, `Network`, and `ScannedChannel`. |
| `driver-use` | Code that starts an existing hardware backend but does not implement one. | `Builder`, `Futures`, `NcpHandle`, `WeakNcpHandle`, `Error`, and `RouteError`. |
| `driver` | Hardware backend crates. | Everything from `driver-use`, plus `Backend`, `Driver`, `EventTranslator`, `bridge`, `Datagram`, `Metadata`, `Event`, `FoundNetwork`, `Network`, `ScannedChannel`, and protocol re-export modules. |

`driver` includes `driver-use`, so backend crates usually enable only `driver`. Application crates
that only need to construct a backend should enable `driver-use`, while coordinator crates should
enable `coordinator`.

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

### Starting an Existing Driver

Enable `driver-use` when your code owns a concrete backend type and needs to configure and start it.
This is the feature for integration code that wires a hardware crate into the coordinator runtime.

```toml
[dependencies]
apis-saltans-hw = { version = "0.7", features = ["driver-use"] }
```

A backend that implements `Builder` can be started from its hardware event stream. The returned
`Futures` value contains:

- `dependencies`: futures that drive the hardware event bridge and event translator.
- `driver`: a future that initializes the backend driver and returns it plus the translated
  event receiver.

All dependency futures must be spawned, or otherwise polled, before spawning or awaiting `driver`.
Starting `driver` first can leave backend initialization waiting for event infrastructure that is
not running yet.

```rust,ignore
use apis_saltans_hw::Builder;

let futures = MyBackend::new(endpoints)?.start(hw_events)?;

for dependency in futures.dependencies {
    tokio::spawn(dependency);
}

let (driver, events) = futures.driver.await?;
```

The returned `events` receiver is intended to be passed to coordinator startup code. If integration
code also needs to name or inspect event payload types directly, enable `coordinator` or `driver` as
well so `Event` and its related data types are re-exported at the crate root.

### Implementing a Driver

Enable `driver` in hardware backend crates. It includes the startup API and exposes the traits used
to implement a backend:

```toml
[dependencies]
apis-saltans-hw = { version = "0.7", features = ["driver"] }
```

Driver crates typically implement:

- `Backend` for the backend's associated hardware event, translator message, and translator type.
- `Builder` to construct a configured backend from coordinator endpoint descriptors.
- `Builder::init` to start the backend and return its driver together with the translated event
  receiver.
- `Driver` on the NCP command actor.
- `EventTranslator` to convert backend events into common `Event` values.

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

### `Builder`

`Builder` constructs a configured backend from the endpoint descriptors exposed by the coordinator
and starts it from a hardware event stream. It relies on the associated types from `Backend` when
creating the bridge and event translator futures that connect hardware-specific events to the
crate-level event model.

### `Futures`

`Futures` contains the initialization future and the dependency futures that drive the bridge and
event translator tasks. Spawn or otherwise poll all dependency futures before spawning or awaiting
the `driver` future. Backend-specific startup state remains internal to the `Builder`
implementation.

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
