# apis-saltans-hw

Hardware abstraction traits and data types for Zigbee network co-processor (NCP) drivers.

This crate separates coordinator logic from concrete hardware backends. A backend implements the
`Driver` trait; coordinator code receives an `NcpHandle` and uses the `Ncp` trait to send commands
to the driver actor. Backends own their event translation and startup wiring.

## Features

No default features are enabled. Pick the feature that matches the role of the crate that depends on
`apis-saltans-hw`.

| Feature | Intended user | Public API |
| --- | --- | --- |
| `coordinator` | Coordinator and application code that already has a running `NcpHandle`. | `Ncp`, `Driver`, `NcpHandle`, `WeakNcpHandle`, `HwResponse`, `Error`, `RouteError`, `Clusters`, `Datagram`, `Metadata`, `Event`, `FoundNetwork`, `Network`, and `ScannedChannel`. |
| `driver` | Hardware backend crates. | `Driver`, `NcpHandle`, `WeakNcpHandle`, `HwResponse`, `Error`, `RouteError`, `Clusters`, `Datagram`, `Metadata`, `Event`, `FoundNetwork`, `Network`, `ScannedChannel`, and protocol re-export modules. |

Backend crates should enable `driver`. Coordinator crates should enable `coordinator`.

`Driver` is shared by both features. This lets integration crates name or re-export the driver
contract without enabling the driver-only protocol re-export modules.

### API Changes

The driver API now consists of the shared `Driver` trait and common hardware types. The former
`Backend` and `EventTranslator` traits and the `bridge` channel helper have been removed. Backend
crates should define any hardware-specific configuration and event message types themselves,
translate incoming events into `Event` values in their own runtime, and use Tokio channels directly
when channel forwarding is needed.

### Using the Coordinator API

Enable `coordinator` when your code receives an `NcpHandle` from startup code and needs to send
commands to the NCP actor.

```toml
[dependencies]
apis-saltans-hw = { version = "0.11", features = ["coordinator"] }
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
networks, reading local endpoint descriptors, allowing joins, resolving addresses, requesting
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

Enable `driver` in hardware backend crates. It exposes the common data types and protocol re-export
modules used to implement a backend:

```toml
[dependencies]
apis-saltans-hw = { version = "0.11", features = ["driver"] }
```

Driver crates implement every `Driver` method on the NCP command actor, including the required
`get_endpoints()` method that reports the NCP's local application endpoints.

Backend startup is owned by the backend crate. It should initialize the concrete driver, translate
hardware events into common `Event` values, and pass the resulting `NcpHandle` plus `Event` receiver
to coordinator startup code. The hardware API intentionally does not impose a translator trait or
channel-bridge helper on that runtime.

The `driver` feature also exposes protocol crate re-export modules:

```rust
use apis_saltans_hw::{aps, core, nwk, zdp};
```

These modules re-export `zb-aps`, `zb-core`, `zb-nwk`, and `zb-zdp` respectively. They are a
convenience for driver crates: public APIs can refer to the protocol types through
`apis_saltans_hw::core::...`, `apis_saltans_hw::aps::...`, and the other re-export modules instead
of adding direct dependencies on every protocol crate.

## Main Traits

### `Driver`

`Driver` is the implementor-facing command API. The sealed actor runtime receives internal
`Message` values and dispatches them to the corresponding `Driver` methods.

Every driver must implement `get_endpoints()` and return one complete
`zdp::SimpleDescriptor` for each application endpoint exposed by the NCP. Descriptors include the
endpoint ID, profile ID, device ID, application version, and input/output cluster lists. The
coordinator treats this as the authoritative local endpoint set when answering ZDP match descriptor
requests and when matching clusters for bindings.

`Driver::transmit(...)` starts the operation and returns an `HwResponse`. Drivers can construct the
response with `HwResponse::new(future)`. The future must be `Send + 'static`, resolve to
`Result<(), E>`, and use an error type convertible into `apis_saltans_hw::Error`. The actor forwards
the response without waiting for the inner future to finish.

Transmission uses one method:

```rust
transmit(destination, datagram)
```

The `Destination` describes the APS target, and the `Datagram` contains APS profile/cluster metadata
plus the serialized application payload. Drivers must also read
`Metadata::aps_acknowledgement()` and enable or suppress APS acknowledgement and retries for that
transmission. `Metadata::new(...)` enables acknowledgement by default; callers such as the OTA
Upgrade server disable it for page-mode block responses.

Local endpoint discovery uses:

```rust
get_endpoints()
```

It returns `Box<[zdp::SimpleDescriptor]>`. The NCP must return every endpoint it advertises; callers
no longer construct or pass a separate descriptor list to the coordinator.

### `Ncp`

`Ncp` is the caller-facing proxy trait implemented for `NcpHandle`. It sends commands to the driver
actor through a Tokio MPSC channel and waits for the one-shot response associated with each command.
`get_endpoints()` returns the same local simple descriptors exposed by the driver.

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

### Local Endpoint Descriptors

`Ncp::get_endpoints()` retrieves the full simple descriptors supplied by the driver:

```rust,no_run
use apis_saltans_hw::{Ncp, NcpHandle};

async fn inspect_local_endpoints(ncp: &NcpHandle) -> Result<(), apis_saltans_hw::Error> {
    for descriptor in ncp.get_endpoints().await? {
        println!(
            "endpoint {}: profile {:#06x}, device {:#06x}",
            descriptor.endpoint_id(),
            descriptor.profile_id(),
            descriptor.device_id(),
        );
    }

    Ok(())
}
```

Use `SimpleDescriptor::input_clusters()` and `SimpleDescriptor::output_clusters()` to inspect the
raw cluster IDs. The standalone `Clusters` helper remains available for code that needs a compact
set of validated `zb_core::Cluster` values, but it is not the return type of `get_endpoints()`.
