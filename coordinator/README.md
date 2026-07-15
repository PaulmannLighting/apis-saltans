# apis-saltans-coordinator

High-level Zigbee coordinator API built on top of [`apis-saltans-hw`](../hw).

This crate starts the coordinator-side transport actors and exposes small traits for Zigbee
operations. It no longer owns device discovery state or binding policy. Applications receive
network events, decide what discovery and binding work they need, and call the provided traits to
perform the individual ZDP/ZCL operations.

For the internal actor graph and message routing details, see [ARCHITECTURE.md](./ARCHITECTURE.md).

## What You Get

Public API exports:

- coordinator handle:
  - `Coordinator`
- low-level transport traits:
  - `Zcl`
  - `Zdp`
- composed ZDP traits:
  - `Node`
  - `Endpoints`
  - `Binding`
- cluster traits:
  - `OnOff`
  - `ColorControl`
  - `Level`
  - `Attributes`
- joining control:
  - `Joining`
- hardware/NCP helper traits:
  - `AddressTranslation`
  - `LocalNode`
  - `Routing`
  - `Scanning`
- attribute helper aliases:
  - `ReadAttributeResult<T>`
  - `WriteAttributeResult`
- scan result types:
  - `FoundNetwork`
  - `ScannedChannel`
- event types:
  - `Event`
  - `Network`
  - `NetworkError`
  - `Device`
- error type:
  - `Error`

## Coordinator Lifecycle

`Coordinator::start(...)` is synchronous and starts three internal tasks:

- the ZCL transceiver
- the ZDP transceiver
- the hardware-event mux

It takes:

- an `NcpHandle` for a running hardware driver actor
- a receiver for translated hardware `zb_hw::Event` values
- a sender for outbound coordinator `Event` values
- the local endpoint descriptors that the coordinator should advertise through ZDP match
  descriptor handling

```rust,no_run
use apis_saltans_coordinator::{Coordinator, Event};
use tokio::sync::mpsc::{Receiver, Sender};
use zb_hw::NcpHandle;
use zb_zdp::SimpleDescriptor;

fn init(
    ncp: NcpHandle,
    hw_events: Receiver<zb_hw::Event>,
    app_events: Sender<Event>,
    endpoints: &[SimpleDescriptor],
) -> Result<Coordinator, zb_hw::Error> {
    Coordinator::start(ncp, hw_events, app_events, endpoints)
}
```

The crate does not persist a device table. Store the `FullAddress` values received in
`Event::Device` if your application needs a device registry. The `AddressTranslation` trait can ask
the NCP to resolve addresses, but persistence and cache policy remain application-owned.

## Trait-Based API

The API is intentionally trait-based. Import the traits you use so extension methods are available
on `Coordinator`.

```rust,no_run
use apis_saltans_coordinator::{
    AddressTranslation, Attributes, Binding, ColorControl, Coordinator, Endpoints, Joining, Level,
    LocalNode, Node, OnOff, Routing, Scanning, Zcl, Zdp,
};
```

The `Coordinator` implements `Zcl`, `Zdp`, `Joining`, `AddressTranslation`, `LocalNode`, `Routing`,
and `Scanning` directly. Discovery, binding, cluster, and attribute traits are blanket
implementations over the raw ZCL/ZDP traits, so they are available on the coordinator without a
separate manager object.

## Events

The application supplies the event channel when starting the coordinator. Events are pushed to that
channel directly; there is no subscription API or internal network-manager fan-out.

```rust,no_run
use apis_saltans_coordinator::{Device, Event, Network};

async fn receive_events(mut events: tokio::sync::mpsc::Receiver<Event>) {
    while let Some(event) = events.recv().await {
        match event {
            Event::Network(Network::Up) => println!("network up"),
            Event::Network(Network::Down) => println!("network down"),
            Event::Network(Network::Opened) => println!("network opened"),
            Event::Network(Network::Closed) => println!("network closed"),
            Event::Network(Network::Error(error)) => println!("network error: {error:?}"),
            Event::Device(Device::Joined(address)) => println!("joined: {address}"),
            Event::Device(Device::Rejoined { address, secured }) => {
                println!("rejoined: {address}, secured={secured}");
            }
            Event::Device(Device::Left(address)) => println!("left: {address}"),
            Event::Device(Device::Announced(address)) => println!("announced: {address}"),
            Event::Zcl { src_address, aps_frame } => {
                println!("unsolicited ZCL from {src_address}: {aps_frame:?}");
            }
            Event::Zdp { src_address, aps_frame } => {
                println!("unsolicited ZDP from {src_address}: {aps_frame:?}");
            }
        }
    }
}
```

`Event::Zcl` and `Event::Zdp` are emitted only for inbound frames that do not match an outstanding
request. Request/response traffic is consumed by the relevant `communicate(...)` call.

## Joining Control

`Joining` opens the network for joins through the hardware stack.

```rust,no_run
use std::time::Duration;
use apis_saltans_coordinator::Joining;

async fn allow_joins(api: &impl Joining) -> Result<Duration, apis_saltans_coordinator::Error> {
    api.allow_joining(Duration::from_secs(60)).await
}
```

The return value is the effective duration accepted by the hardware.

## Hardware Helpers

These traits expose NCP operations that are useful when building application-owned coordinator
services.

### Local Node

```rust,no_run
use apis_saltans_coordinator::LocalNode;
use zb_core::IeeeAddress;

async fn local_info(api: &impl LocalNode) -> Result<(u16, IeeeAddress), apis_saltans_coordinator::Error> {
    let pan_id = api.get_pan_id().await?;
    let ieee = api.get_ieee_address().await?;
    Ok((pan_id, ieee))
}
```

### Address Translation

```rust,no_run
use apis_saltans_coordinator::AddressTranslation;
use zb_core::IeeeAddress;

async fn refresh_short_id(
    api: &impl AddressTranslation,
    ieee: IeeeAddress,
) -> Result<u16, apis_saltans_coordinator::Error> {
    api.ieee_address_to_short_id(ieee).await
}
```

Use this to consult the NCP's address table. Applications should still decide whether and how to
cache the result.

### Scanning

```rust,no_run
use apis_saltans_coordinator::{FoundNetwork, Scanning};

async fn scan(api: &impl Scanning) -> Result<Vec<FoundNetwork>, apis_saltans_coordinator::Error> {
    const ALL_CHANNELS: u32 = 0x07fff800;
    const DEFAULT_DURATION: u8 = 5;

    api.scan_networks(ALL_CHANNELS, DEFAULT_DURATION).await
}
```

`scan_networks(...)` returns discovered networks. `scan_channels(...)` returns channel scan
observations.

### Routing

```rust,no_run
use apis_saltans_coordinator::Routing;

async fn request_routes(api: &impl Routing) -> Result<(), apis_saltans_coordinator::Error> {
    const DEFAULT_RADIUS: u8 = 30;

    api.route_request(DEFAULT_RADIUS).await
}
```

## Discovery Building Blocks

Discovery is application-owned. The coordinator provides reusable operations for the standard ZDP
steps, and your application chooses when to run them, how to retry them, and what state to persist.

### Node Descriptor

```rust,no_run
use apis_saltans_coordinator::Node;
use zb_core::short_id::Device;

async fn read_node_descriptor(
    api: &impl Node,
    short_id: Device,
) -> Result<zb_core::node::Descriptor, apis_saltans_coordinator::Error> {
    api.descriptor(short_id, None).await
}
```

### Active Endpoints and Simple Descriptors

```rust,no_run
use apis_saltans_coordinator::Endpoints;
use std::collections::BTreeMap;
use zb_core::Endpoint;
use zb_core::short_id::Device;
use zb_zdp::SimpleDescriptor;

async fn read_endpoint_descriptors(
    api: &impl Endpoints,
    short_id: Device,
) -> Result<BTreeMap<Endpoint, Result<Option<SimpleDescriptor>, apis_saltans_coordinator::Error>>, apis_saltans_coordinator::Error> {
    let endpoints = api.endpoints(short_id).await?;
    Ok(api.descriptors(short_id, endpoints).await)
}
```

`descriptor(...)` returns `Ok(None)` when the response is successful but contains no descriptor.
Non-success ZDP statuses are returned as `Error::Zdp(...)`.

### Binding

`Binding` sends ZDP `BindReq` commands. The crate does not decide which clusters should be bound or
when a device is fully integrated.

```rust,no_run
use apis_saltans_coordinator::Binding;
use zb_core::{Cluster, Endpoint, FullAddress};
use zb_zdp::Destination;

async fn bind_cluster(
    api: &impl Binding,
    address: FullAddress,
    source_endpoint: Endpoint,
    cluster: Cluster,
    destination: Destination,
) -> Result<(), apis_saltans_coordinator::Error> {
    api.bind(address, source_endpoint, cluster, destination).await
}
```

Use `bind_all(...)` when you already have an endpoint-to-clusters map and want a per-endpoint
result map.

## ZCL Cluster Helpers

Cluster helper traits build standard ZCL commands and send them through the `Zcl` transport.
Commands that do not expect an application-level response use `transmit(...)`.

### On/Off

```rust,no_run
use apis_saltans_coordinator::OnOff;
use zb_core::destination::Device as DeviceDestination;
use zb_core::short_id::Device;
use zb_core::{Application, Destination};

async fn switch_on(api: &impl OnOff) -> Result<(), apis_saltans_coordinator::Error> {
    let short_id = Device::try_from(0x1234).expect("valid short address");
    let endpoint = Application::try_from(1).expect("valid endpoint");
    let destination = Destination::from(DeviceDestination::new(short_id, endpoint.into()));

    api.on(destination).await
}
```

The `OnOff` trait provides `on`, `off`, `off_with_effect`, and `toggle`.

### Level

`Level` provides the standard level-control commands:

- `move_to_level`
- `move`
- `step`
- `stop`
- `move_to_level_with_on_off`
- `move_with_on_off`
- `step_with_on_off`
- `stop_with_on_off`
- `move_to_closest_frequency`

### Color Control

```rust,no_run
use apis_saltans_coordinator::ColorControl;
use zb_core::destination::Device as DeviceDestination;
use zb_core::short_id::Device;
use zb_core::units::{Deciseconds, Mireds};
use zb_core::{Application, Destination};
use zb_zcl::Options;

async fn set_color_temperature(
    api: &impl ColorControl,
) -> Result<(), apis_saltans_coordinator::Error> {
    let short_id = Device::try_from(0x1234).expect("valid short address");
    let endpoint = Application::try_from(1).expect("valid endpoint");
    let destination = Destination::from(DeviceDestination::new(short_id, endpoint.into()));

    api.move_to_color_temperature(
        destination,
        Mireds::try_from(250).expect("valid color temperature"),
        Deciseconds::new(10).expect("valid transition time"),
        Options::empty(),
    )
    .await
}
```

`ColorControl` provides `move_to_xy` and `move_to_color_temperature`.

## Generic Attribute Access

`Attributes` provides typed ZCL global attribute operations.

The target is a `zb_core::destination::Device`, which contains the short address and endpoint.
Build or look this up from your own discovery state before calling the trait.

### Reads

```rust,no_run
use apis_saltans_coordinator::{Attributes, ReadAttributeResult};
use zb_core::destination::Device as DeviceDestination;
use zb_core::short_id::Device;
use zb_core::Application;
use zb_zcl::general::basic::readable::Id as BasicReadableId;

async fn read_basic(
    api: &impl Attributes,
    short_id: Device,
) -> Result<Box<[ReadAttributeResult<BasicReadableId>]>, apis_saltans_coordinator::Error> {
    let endpoint = Application::try_from(1).expect("valid endpoint");
    let device = DeviceDestination::new(short_id, endpoint.into());

    api.read(
        device,
        [
            BasicReadableId::ModelIdentifier,
            BasicReadableId::ManufacturerName,
        ],
    )
    .await
}
```

### Writes

```rust,no_run
use apis_saltans_coordinator::Attributes;
use zb_core::destination::Device as DeviceDestination;
use zb_core::short_id::Device;
use zb_core::types::String;
use zb_core::Application;
use zb_zcl::general::basic::writable::Attribute as BasicWritable;

async fn write_location(
    api: &impl Attributes,
    short_id: Device,
) -> Result<(), apis_saltans_coordinator::Error> {
    let endpoint = Application::try_from(1).expect("valid endpoint");
    let device = DeviceDestination::new(short_id, endpoint.into());
    let location = String::<16>::try_from("Living Room").expect("fits");

    let result = api
        .write(device, [BasicWritable::LocationDescription(location)])
        .await?;

    let _per_attribute_status = result;
    Ok(())
}
```

### Reporting

Use `configure_reporting(...)` with generated ZCL `Reportable` values. The ZCL attribute value
supplies cluster/profile/manufacturer and data type metadata; the coordinator only transports the
request.

## Raw Transports

Use `Zcl::transmit(...)` for native cluster commands that do not expect a response, and
`Zcl::communicate(...)` for commands implementing `ExpectResponse<zb_zcl::Cluster>`.

Use `Zdp::communicate(...)` for ZDP requests implementing `ExpectResponse<zb_zdp::Command>`.
The composed traits above are thin wrappers over these raw transports.

## Error Model

Most APIs return `apis_saltans_coordinator::Error`:

- `Hardware(zb_hw::Error)`
- `SendError`
- `ReceiveError(RecvError)`
- `Timeout(Elapsed)`
- `InvalidResponseType(String)`
- `UnknownDevice(IeeeAddress)`
- `InvalidApplicationEndpoint(u8)`
- `DurationOutOfBounds(Duration)`
- `Zcl(Result<zb_zcl::Status, u8>)`
- `Zdp(Result<zb_zdp::Status, u8>)`

ZCL and ZDP status responses preserve known status enums and raw unknown status bytes.

## Runtime Configuration

Behavior is configurable through environment variables:

- `ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE`
- `ZIGBEE_COORDINATOR_ZCL_RESPONSE_TIMEOUT_SECS`
- `ZIGBEE_COORDINATOR_ZDP_RESPONSE_TIMEOUT_SECS`

Retry behavior for discovery or binding is intentionally not configured here anymore. Applications
that build discovery or binding workflows should apply their own retry and persistence policy.
