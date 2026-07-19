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
| `coordinator` | Coordinator and application code that already has a running `NcpHandle`. | `Ncp`, `NcpHandle`, `WeakNcpHandle`, `HwResponse`, `Error`, `RouteError`, `Clusters`, `Datagram`, `Metadata`, `Event`, `FoundNetwork`, `Network`, and `ScannedChannel`. |
| `driver` | Hardware backend crates. | `Backend`, `Driver`, `EventTranslator`, `bridge`, `NcpHandle`, `WeakNcpHandle`, `HwResponse`, `Error`, `RouteError`, `Clusters`, `Datagram`, `Metadata`, `Event`, `FoundNetwork`, `Network`, `ScannedChannel`, and protocol re-export modules. |

Backend crates should enable `driver`. Coordinator crates should enable `coordinator`.

### Using the Coordinator API

Enable `coordinator` when your code receives an `NcpHandle` from startup code and needs to send
commands to the NCP actor.

```toml
[dependencies]
apis-saltans-hw = { version = "0.10", features = ["coordinator"] }
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
networks, reading local endpoint cluster sets, allowing joins, resolving addresses, requesting
routes, and transmitting serialized `Datagram` values to `zb_core::Destination` targets.

`Ncp::transmit(...)` is intentionally two-stage. Awaiting the method sends the request to the driver
actor and returns an opaque `HwResponse`. Await that response to observe the driver's actual
transmission result:

```rust,ignore
let response = ncp.transmit(destination, datagram).await?;
response.await?;
```

An error from the first await means the actor command could not be delivered or the driver could not
start the operation. An error from the second await is the deferred hardware transmission failure.
`HwResponse` hides whether a backend uses a channel, an I/O future, or another completion mechanism.

The common `Error` type implements `std::error::Error`. Backend-specific `Implementation` failures
are retained as an error source; closed actor channels are represented by the payload-free
`DriverSend` and `DriverRecv` variants.

### Implementing a Driver

Enable `driver` in hardware backend crates. It exposes the traits and common data types used to
implement a backend:

```toml
[dependencies]
apis-saltans-hw = { version = "0.10", features = ["driver"] }
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

`Driver::transmit(...)` starts the operation and returns an `HwResponse`. Drivers can construct the
response with `HwResponse::new(future)`. The future must be `Send + 'static`, resolve to
`Result<(), E>`, and use an error type convertible into `apis_saltans_hw::Error`. The actor forwards
the response without waiting for the inner future to finish.

Transmission uses one method:

```rust
transmit(destination, datagram)
```

The `Destination` describes the APS target, and the `Datagram` contains APS profile/cluster metadata
plus the serialized application payload.

Local endpoint cluster discovery uses:

```rust
get_endpoints()
```

It returns a map keyed by `zb_core::Application` endpoint ID with `Clusters` values. Each `Clusters`
value contains the input and output `zb_core::Cluster` sets the driver has registered on that local
application endpoint.

### `EventTranslator`

`EventTranslator` converts hardware-specific event messages into common `Event` values such as
network state changes, device joins/leaves with `FullAddress`, route errors, and received APS data.

### `Ncp`

`Ncp` is the caller-facing proxy trait implemented for `NcpHandle`. It sends commands to the driver
actor through a Tokio MPSC channel and waits for the one-shot response associated with each command.
`get_endpoints()` returns the same local endpoint cluster map exposed by the driver.

Most proxy methods await the driver result before returning. `Ncp::transmit(...)` instead returns
the driver's `HwResponse` immediately after actor handoff, allowing coordinator layers to compose
hardware completion with their own protocol-response futures without depending on the driver's
completion mechanism.

### `HwResponse`

`HwResponse` is an opaque future with output `Result<(), Error>`. It owns the driver-provided
completion future and can be moved into a higher-level response object. Await it exactly like any
other future:

```rust,ignore
let response = ncp.transmit(destination, datagram).await?;
response.await?;
```

Dropping an `HwResponse` drops its inner future. Whether that cancels an operation depends on the
driver's future and hardware backend; the abstraction does not promise cancellation.

### `Clusters`

`Clusters` describes the input and output cluster IDs advertised by one local application endpoint.
It is used as the value in the `get_endpoints()` map:

```rust,no_run
use std::collections::BTreeMap;

use apis_saltans_hw::{Clusters, Ncp, NcpHandle};
use zb_core::Application;

async fn local_clusters(
    ncp: &NcpHandle,
) -> Result<BTreeMap<Application, Clusters>, apis_saltans_hw::Error> {
    ncp.get_endpoints().await
}
```

Use `Clusters::input()` and `Clusters::output()` to inspect the endpoint's cluster sets.
