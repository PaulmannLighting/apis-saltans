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
- hardware driver trait:
  - `Driver` (re-exported from `apis-saltans-hw`)
- low-level transport traits:
  - `Zcl`
  - `Zdp`
- OTA server API:
  - `Ota`
  - `OtaImage`
  - `OtaTarget`
  - `OtaMessage`
- deferred response futures:
  - `TransmissionResponse`
  - `CommunicationResponse<T, U>`
  - `ZclResponse<T>`
  - `ZdpResponse<T>`
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

Commands without a protocol response return `TransmissionResponse`, which adapts the hardware
response to the coordinator's `Error` type.

The `Driver` re-export lets integration crates import the coordinator API and implement the NCP
driver contract from one dependency. Hardware-specific event translation and startup wiring remain
the backend's responsibility.

## Coordinator Lifecycle

`Coordinator::start(...)` is synchronous and starts four internal tasks:

- the ZCL transceiver
- the ZDP transceiver
- the OTA Upgrade server
- the hardware-event mux

It takes:

- an `NcpHandle` for a running hardware driver actor
- the coordinator's Zigbee node descriptor
- a receiver for translated hardware `zb_hw::Event` values
- a sender for outbound coordinator `Event` values

The NCP driver must implement `zb_hw::Driver::get_endpoints()` and return a complete
`zb_zdp::SimpleDescriptor` for every local application endpoint. The coordinator retrieves these
descriptors through `zb_hw::Ncp::get_endpoints()` when it needs them; endpoint descriptors are no
longer passed to `Coordinator::start(...)`.

```rust,no_run
use apis_saltans_coordinator::{Coordinator, Event};
use tokio::sync::mpsc::{Receiver, Sender};
use zb_core::node::Descriptor;
use zb_hw::NcpHandle;

fn init(
    ncp: NcpHandle,
    descriptor: Descriptor,
    hw_events: Receiver<zb_hw::Event>,
    app_events: Sender<Event>,
) -> Result<Coordinator, zb_hw::Error> {
    Coordinator::start(ncp, descriptor, hw_events, app_events)
}
```

When a remote device sends `MatchDescReq`, the ZDP transceiver asks the NCP for its current endpoint
descriptors and builds `MatchDescRsp` from matching descriptors. If the NCP cannot provide them, the
request cannot be answered.

The crate does not persist a device table. Store the `FullAddress` values received in
`Event::Device` if your application needs a device registry. The `AddressTranslation` trait can ask
the NCP to resolve addresses, but persistence and cache policy remain application-owned.

## OTA Upgrade Server

The coordinator owns an OTA Upgrade (`0x0019`) server. Parse a complete Zigbee OTA file into an
`OtaImage`, select the device endpoint and application profile with `OtaTarget`, and call
`Ota::update`. The image parser validates the OTA identifier, version, declared header and file
sizes, optional destination, and hardware-version range before the image can be scheduled.

```rust,no_run
use apis_saltans_coordinator::{Coordinator, Ota, OtaImage, OtaTarget};
use bytes::Bytes;
use zb_core::destination::Device;
use zb_core::Profile;

async fn offer_update(
    coordinator: &Coordinator,
    destination: Device,
    ota_file: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let image = OtaImage::parse(Bytes::from(ota_file))?;
    let target = OtaTarget::new(destination, Profile::ZigbeeHomeAutomation);
    coordinator.update(target, image).await?;
    Ok(())
}
```

Scheduling sends a unicast Image Notify automatically. Incoming Query Next Image, Query Specific
File, Image Block, Image Page, and Upgrade End requests are then consumed by the server rather than
published as general `Event::Zcl` values. The server selects the scheduled image, validates device
and file metadata, streams blocks (including paced page responses), preserves or advances ZCL
transaction numbers as required, and emits the appropriate command or default response. Scheduling
another image for the same device endpoint replaces its current offer.

ZCL returns the deferred `HwResponse` for every internally generated OTA command to the OTA actor.
The actor polls ordinary command completions in tracked background tasks, so deferred hardware work
does not block incoming OTA requests. Page-transfer tasks poll their own block responses and stop
the remaining page stream if a transmission fails.

Drivers must honor `zb_hw::Metadata::aps_acknowledgement()`. Normal OTA replies request APS
acknowledgements; block responses generated by Image Page Request disable them as required by ZCL.

## Trait-Based API

The API is intentionally trait-based. Import the traits you use so extension methods are available
on `Coordinator`.

```rust,no_run
use apis_saltans_coordinator::{
    AddressTranslation, Attributes, Binding, ColorControl, Coordinator, Endpoints, Joining, Level,
    LocalNode, Node, OnOff, Routing, Scanning, Zcl, Zdp,
};
```

The `Coordinator` implements `Ota`, `Zcl`, `Zdp`, `Joining`, `AddressTranslation`, `LocalNode`,
`Routing`, and `Scanning` directly. Discovery, binding, cluster, and attribute traits are blanket
implementations over the raw ZCL/ZDP traits, so they are available on the coordinator without a
separate manager object.

## Deferred Responses

Sending is split into two observable stages. The first await queues work on the coordinator actor
and returns a response future. Awaiting that returned future observes the hardware and protocol
result:

- `TransmissionResponse` waits for hardware completion of a ZCL command that has no
  application-level response and converts hardware failures into `Error::Hardware`.
- `ZclResponse<T>` waits for hardware completion, then a correlated ZCL frame, and converts that
  frame to `T`.
- `ZdpResponse<T>` does the same for a correlated ZDP command.
- `CommunicationResponse<Raw, T>` is the generic future behind the two protocol aliases.

Consequently, raw transport and command-helper calls commonly use two awaits:

```rust,ignore
let transmission = api.on(destination).await?;
transmission.await?;

let response = api.communicate(device, request).await?;
let typed_response = response.await?;
```

The first error reports that the request could not be queued or handed off. The second await reports
hardware transmission, response-channel, or typed-conversion failures. Dropping a returned response
future stops driving and observing that future. The coordinator does not promise that dropping it
cancels work already handed to the hardware backend.

`Error` implements `std::error::Error`. Hardware, one-shot receive, and timeout variants retain and
expose their source errors and can be constructed through `From`; the send variant intentionally
discards the failed channel payload.

Higher-level discovery and binding helpers consume both stages internally when they return a final
value. `Groups::list(...)` and `Attributes::configure_reporting(...)` intentionally expose a
`ZclResponse<T>` so callers retain control over when to await the device response.

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
            Event::Device(Device::KeepAlive(device)) => {
                println!("keep-alive from {device}");
            }
            Event::Zcl { src_address, aps_frame } => {
                println!("unsolicited ZCL from {src_address}: {aps_frame:?}");
            }
        }
    }
}
```

`Event::Zcl` is emitted only for inbound frames that do not match an outstanding request.
Request/response traffic is consumed by the relevant `communicate(...)` call.

An APS packet with cluster ID `0x0025` (`Cluster::KeepAlive`) under a supported application profile
is handled before ZCL payload decoding and produces `Device::KeepAlive`. The contained
`zb_core::destination::Device` identifies the sender by its NWK short address and APS source
endpoint. Packets whose source is not an allocated device short address or whose source endpoint is
reserved are logged and dropped instead of producing an event.

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

`LocalNode::get_endpoints()` returns the same boxed slice of `SimpleDescriptor` values supplied by
the NCP. This makes the hardware's endpoint configuration available without maintaining a second
coordinator-owned copy.

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
    api.descriptors(short_id).await
}
```

`descriptor(...)` returns `Ok(None)` when the response is successful but contains no descriptor.
Non-success ZDP statuses are returned as `Error::Zdp(...)`.

`descriptors(...)` first calls `endpoints(...)`. If active endpoint discovery fails, the outer
`Result` is `Err(...)`. If endpoint discovery succeeds, the returned map contains one descriptor
result per endpoint, so callers can keep partial results from endpoints that succeeded.

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

Use `bind_all_to_self(...)` when remote endpoint output clusters should be bound to matching local
coordinator endpoints. The helper reads the coordinator IEEE address and local simple descriptors
through `LocalNode`, intersects each descriptor's input clusters with each remote endpoint's output
clusters, and sends bind requests for matching clusters only.

```rust,no_run
use std::collections::{BTreeMap, BTreeSet};

use apis_saltans_coordinator::Binding;
use zb_core::{Cluster, Endpoint, FullAddress};

async fn bind_matching_clusters_to_coordinator(
    api: &(impl Binding + apis_saltans_coordinator::LocalNode),
    address: FullAddress,
    source_endpoint_clusters: BTreeMap<Endpoint, BTreeSet<Cluster>>,
) -> Result<BTreeMap<Endpoint, Result<(), apis_saltans_coordinator::Error>>, apis_saltans_coordinator::Error> {
    api.bind_all_to_self(address, source_endpoint_clusters).await
}
```

The outer `Result` reports local coordinator lookup failures. The returned map contains per-source
endpoint bind results for requests that were attempted. If multiple local endpoints can receive
clusters from the same remote source endpoint, later local endpoint results overwrite earlier
results for that source endpoint in the returned map.

Use `bind_all(...)` when you already know the exact ZDP binding destination and want to bind an
endpoint-to-clusters map to that destination.

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

    api.on(destination).await?.await
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
    .await?
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

Use `Zcl::transmit(...)` for native cluster commands that do not expect an application-level
response. Its first await returns a `TransmissionResponse`; await that value to confirm hardware
completion. It wraps the opaque `zb_hw::HwResponse`, so coordinator users receive the local `Error`
type without depending on the driver's completion mechanism.

Use `Zcl::communicate(...)` for commands implementing `ExpectResponse<zb_zcl::Cluster>`. Its first
await returns `ZclResponse<T::Response>`. Awaiting that response confirms transmission, waits for a
correlated ZCL frame, and converts the frame to the declared response type.

Use `Zdp::communicate(...)` for ZDP requests implementing `ExpectResponse<zb_zdp::Command>`. It
returns the equivalent `ZdpResponse<T::Response>`. The composed traits above are thin wrappers over
these raw transports; most of them await the deferred response internally.

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

For deferred operations, an error can occur at either await boundary. Queue and actor handoff
errors occur while obtaining the response future. Hardware, receive-channel, and conversion errors
occur while awaiting that response future.

## Runtime Configuration

Behavior is configurable through environment variables:

- `ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE`

Deferred response futures do not impose a deadline. Applications that require one can wrap the
second await with `tokio::time::timeout` and select a timeout policy appropriate to the operation.

Retry behavior for discovery or binding is intentionally not configured here anymore. Applications
that build discovery or binding workflows should apply their own retry and persistence policy.
