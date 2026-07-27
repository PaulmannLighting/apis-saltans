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
| `types` | Code that only exchanges common hardware values. | Opaque handles, errors, events, and typed scan parameters and results. |
| `coordinator` | Coordinator and application code that already has a running `NcpHandle`. | Shared types plus the caller-facing `Ncp` trait. |
| `driver` | Hardware backend crates. | Shared types, the implementor-facing `Driver` trait, and protocol re-export modules. |
| `serde` | Code that serializes supported hardware values. | Serialization for operation, scan, and network descriptor types; also enables `types`. |

Backend crates should enable `driver`. Coordinator crates should enable `coordinator`.

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
apis-saltans-hw = { version = "0.13", features = ["coordinator"] }
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
routes, and transmitting `zb_aps::Data<bytes::Bytes>` frames to `zb_core::Destination` targets.

`Ncp::transmit(...)` hands a `zb_aps::Data<bytes::Bytes>` frame to the driver actor and waits for
the backend to accept it:

```rust,ignore
ncp.transmit(destination, frame).await?;
```

For frames requesting an APS acknowledgement, backends later publish
`Event::Aps(ApsEvent::Ack(counter))` or
`Event::Aps(ApsEvent::Nak { sequence: counter, error })`. The `counter` is read from the transmitted
APS frame.

The common `Error` type implements `std::error::Error`. Backend-specific failures retain their
source, while a stopped driver actor is represented by `Error::ActorUnavailable`.

### Hardware Events

Hardware events are grouped by their protocol responsibility:

```rust,ignore
use apis_saltans_hw::{ApsEvent, DeviceEvent, Event, NetworkEvent};

let network_up = Event::Network(NetworkEvent::Up);
let device_joined = Event::Device(DeviceEvent::Joined(address));
let acknowledged = Event::Aps(ApsEvent::Ack(counter));
```

`NetworkEvent` reports network state and route errors, `DeviceEvent` reports device membership
changes, and `ApsEvent` reports received frames and acknowledged transmission results.

### Implementing a Driver

Enable `driver` in hardware backend crates. It exposes the common data types and protocol re-export
modules used to implement a backend:

```toml
[dependencies]
apis-saltans-hw = { version = "0.13", features = ["driver"] }
```

Driver crates implement every `Driver` method on the NCP command actor, including the required
`get_endpoints()` method that reports the NCP's local application endpoints.

Convert a driver into its opaque handle and driving future with `Driver::into_actor(...)`:

```rust,ignore
use std::num::NonZeroUsize;

use apis_saltans_hw::Driver;

let (ncp, actor) = driver.into_actor(NonZeroUsize::new(32).expect("non-zero actor capacity"));
tokio::spawn(actor);
```

The returned future is deliberately not spawned by `apis-saltans-hw`; backend startup owns the
runtime and must spawn or otherwise continuously poll it.

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

`Driver` is the implementor-facing command API. The actor runtime receives internal
`Message` values and dispatches them to the corresponding `Driver` methods.

Every driver must implement `get_endpoints()` and return one complete
`zdp::SimpleDescriptor` for each application endpoint exposed by the NCP. Descriptors include the
endpoint ID, profile ID, device ID, application version, and input/output cluster lists. The
coordinator treats this as the authoritative local endpoint set when answering ZDP match descriptor
requests and when matching clusters for bindings.

`Driver::transmit(...)` receives a complete `aps::Data<bytes::Bytes>` frame. Returning success means
the hardware backend accepted the frame. For acknowledged transmissions, the backend later emits
`ApsEvent::Ack` or `ApsEvent::Nak` with the frame's `u8` APS counter.

Transmission uses one method:

```rust
transmit(destination, frame)
```

The `Destination` describes the NWK target. The APS data frame contains the destination endpoint,
cluster, profile, source endpoint, APS counter, control flags, and serialized application payload.

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

Every proxy method creates and awaits a response channel. `Ncp::transmit(...)` returns after backend
acceptance; eventual APS completion returns through the hardware event stream and is identified by
the APS counter already carried by the frame.

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
raw cluster IDs.
