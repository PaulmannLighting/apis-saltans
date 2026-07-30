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
- OTA server API:
  - `Ota`
  - `ParseImage`
  - `OtaBaseHeaderBytes`
  - `OtaFieldControl`
  - `OtaHeader`
  - `OtaHeaderString`
  - `OtaImage`
  - `OtaMessage`
  - `OtaUpdateError`
  - `OtaUpdateResult`
- deferred response futures:
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
  - `Channel`
  - `ChannelMask`
  - `ScanDuration`
  - `FoundNetwork`
  - `NetworkDescriptor`
  - `ScannedChannel`
- event types:
  - `Event`
  - `Network`
  - `NetworkError`
  - `Device`
- error type:
  - `Error`

Unicast commands without a protocol response await acknowledged APS completion directly. Every
transmission waits for hardware-backend acceptance; group, broadcast, and other unacknowledged
transmissions complete at that point. Hardware driver implementations use the separate
`apis-saltans-hw` `driver` feature.

## Coordinator Lifecycle

`Coordinator::start(...)` is synchronous and starts six internal tasks:

- the APS transceiver
- the ZCL transceiver
- the ZDP transceiver
- the OTA API inbox forwarder
- the OTA Upgrade server
- the hardware-event mux

It takes:

- an `NcpHandle` for a running hardware driver actor
- the coordinator's Zigbee node descriptor
- a receiver for translated hardware `zb_hw::Event` values
- a sender for outbound coordinator `Event` values

Application-event delivery is non-blocking. When that sender's channel is full or closed, the
coordinator drops the new event and logs the condition so application backpressure cannot stall
hardware-event routing or the protocol actors. Applications should drain the event channel
promptly and treat it as a lossy notification stream rather than durable state.

By default, the OTA server runs at most `ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE` concurrent
destination transfer tasks. Use `Coordinator::start_with_ota_update_task_limit(...)` to select a
different limit. Each task lasts for the complete OTA exchange and owns its transmission
operations. Replacing the update for a destination reuses its task. A new destination is rejected
through its completion future if no task slot is available.

The NCP driver must implement `zb_hw::Driver::get_endpoints()` and return a complete
`zb_zdp::SimpleDescriptor` for every local application endpoint. The coordinator retrieves these
descriptors for local-node queries and ZDP match handling; endpoint descriptors are no longer
passed to `Coordinator::start(...)`. ZCL does not use the descriptors to select an endpoint.
Applications select the local source endpoint explicitly for every ZCL operation.
The OTA server also uses `Driver::short_id_to_ieee_address()` to validate every request against the
full device identity supplied when its update was scheduled.

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

The coordinator also serves local Active Endpoint and Simple Descriptor discovery from those NCP
descriptors. It answers single-device Network Address and IEEE Address requests through the NCP's
address-translation operations and responds to matching System Server Discovery requests from the
server mask in its node descriptor. Extended address discovery returns `NOT_SUPPORTED` because the
hardware abstraction does not expose associated-device enumeration. Power Descriptor discovery
returns `NO_DESCRIPTOR` because coordinator startup does not configure a power descriptor.

The crate does not persist a device table. Store the `FullAddress` values received in
`Event::Device` if your application needs a device registry. The `AddressTranslation` trait can ask
the NCP to resolve addresses, but persistence and cache policy remain application-owned.

## OTA Upgrade Server

The coordinator owns an OTA Upgrade (`0x0019`) server. Open a complete Zigbee OTA file as a
seekable reader, parse it into an `OtaImage`, select the device endpoint, and call `Ota::update`.
OTA traffic uses the Zigbee Home Automation profile. The image parser validates the identifier,
version, null-terminated ASCII header string, declared header and file sizes, optional destination,
and hardware-version range before the image can be scheduled.

`OtaImage` keeps its parsed `OtaHeader` in memory and retains the supplied
`Read + Seek + Send + 'static` source for payload reads; it does not copy the payload into an image
buffer. Seeking is required because clients can request blocks out of order or retry an earlier
file offset. When the image is scheduled, a dedicated transfer task takes ownership of the reader.
Lightweight transfer handles send range requests to that task, which serializes access to the file
cursor without a shared mutex.

`ParseImage` is implemented for `std::fs::File`. Its `parse(self)` method composes default methods
for seeking to the image start, reading the fixed and optional header sections, determining the
source length, and positioning the retained source at the payload. Implementations for other
reader types can override any of these stages while retaining the default parser workflow.

```rust,no_run
use apis_saltans_coordinator::{Coordinator, Ota, ParseImage};
use std::fs::File;
use std::path::Path;
use zb_aps::apsde::IndividualEndpoint;
use zb_core::{Application, Endpoint, FullAddress};

async fn offer_update(
    coordinator: &Coordinator,
    target: FullAddress,
    target_endpoint: IndividualEndpoint,
    ota_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let image = File::open(ota_path)?.parse()?;
    let source_endpoint = IndividualEndpoint::new(Endpoint::from(
        Application::try_from(1).expect("valid local endpoint"),
    ))
    .expect("application endpoints are individual");
    coordinator
        .update(target, target_endpoint, source_endpoint, image)
        .await?;
    Ok(())
}
```

Scheduling sends a unicast Image Notify automatically. Incoming Query Next Image, Query Specific
File, Image Block, Image Page, and Upgrade End requests are then consumed by the server rather than
published as general `Event::Zcl` values. The server selects the scheduled image, validates device
and file metadata, streams blocks (including paced page responses), preserves or advances ZCL
transaction numbers as required, and emits the appropriate command or default response. Scheduling
another image for the same device endpoint replaces its current offer.

Every update is scheduled with a `FullAddress`, pinning the client's IEEE identity to its current
NWK short address. Before routing an inbound request to that transfer, the OTA server resolves the
request's short address through the NCP and requires it to match the pinned IEEE address. Optional
request-node addresses and an image's `upgrade_file_destination` are checked against the same
identity. A different device that later acquires the short address therefore cannot inherit the
offer.

The OTA subsystem receives those requests through an internal ZCL subscription filtered by a typed
cluster variant, command scope, and direction. ZCL has no OTA-specific routing logic or OTA actor
handle. Its subscription handle is weak, so the subscription does not create an actor-lifetime
cycle. The OTA server creates and registers this subscription on demand when it admits the first
device update. It queues registration before spawning the destination transfer that sends Image
Notify. A lightweight task forwards subscribed frames through a weak sender into the server's
private event inbox. Concurrent destination updates reuse the subscription. After the final
destination transfer finishes, the server stops the forwarding task and explicitly unregisters the
subscription from ZCL; a later update registers a new one. Public OTA API messages and destination
transfer completions are forwarded into that same inbox, so the server awaits one receiver.
Coordinator startup therefore does not wire OTA frame routing itself, and the weak forwarders
cannot keep the server alive after external OTA handles are dropped.

ZCL delivers subscription frames without awaiting channel capacity. If a subscription channel is
full, the current frame continues through application-event routing. Response correlation checks
the expected response direction before subscription delivery, so a correlated response cannot be
consumed by a subscription while unrelated client requests continue to reach it. Closed
subscription channels are removed automatically.

`Ota::update` remains pending for the complete exchange. It returns success after the client sends
a successful Upgrade End Request. Client rejection, image-read failures, terminal transmission
failures, replacement by a newer update, and exhaustion of the configured concurrent update-task
limit return an `OtaUpdateError`; stopping the OTA actor before completion returns the
coordinator's receive error.

The server routes commands to one long-lived task per destination and tracks only those destination
tasks. Each destination task owns its transmission operations and any paced page-transfer
operation. ZCL reply frames copy the request's transaction sequence number; consecutive Image Page
responses advance it for each block.

Normal OTA commands and replies request `TxOptions::ACKNOWLEDGED_TRANSMISSION`, so their send
operations wait for the APS result. Block responses generated by Image Page Request use empty
transmission options as required by ZCL. Those unacknowledged responses complete after the hardware
backend accepts them, allowing the page-transfer task to apply the requested response spacing
before sending the next block.

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

## Transmission and Protocol Responses

`Zcl::transmit(...)` and the command helpers await the acknowledged APS result directly:

```rust,ignore
api.on(destination, source_endpoint).await?;
```

Communication remains split at the protocol boundary:

```rust,ignore
let response = api.communicate(request).await?;
let typed_response = response.await?;
```

The communication method queues the command and returns `ZclResponse<T>` or `ZdpResponse<T>`
without making the protocol actor wait for an APS acknowledgement. Awaiting that response future
first completes the deferred APS transmission and then receives and converts the correlated
protocol response. `CommunicationResponse<Raw, T>` is the generic future behind both aliases.

The coordinator creates a deferred APS response for every transmission. It completes on hardware
rejection or acceptance unless the request's `TxOptions` contain `ACKNOWLEDGED_TRANSMISSION` and
its destination is a unicast device. Such a unicast remains pending for its APS result. Group and
broadcast transmissions never request APS acknowledgements, regardless of that option.
Acknowledged results arrive as hardware
`Event::Apsde(ApsdeEvent::DataConfirm { counter, confirmation })` values and are correlated by the
eight-bit APS counter supplied to the hardware. A successful confirmation resolves the deferred
result; an unsuccessful APS or propagated NWK status becomes
`TransmissionError::Confirmation`. An acknowledged transmission times out after 30 seconds if its
confirmation does not arrive. A network-down event resolves every pending acknowledged
transmission with `TransmissionError::NoRoute`. Dropping a deferred APS result removes its pending
confirmation entry; work already accepted by the hardware is not recalled.

The APS actor allocates counters from the complete 256-value Zigbee counter space. It never
replaces a pending transmission. A counter released by cancellation, timeout, or network loss is
quarantined until its late hardware confirmation arrives, so an old confirmation cannot complete a
new transmission. Quarantine has no time-based expiry because the hardware contract does not
define a maximum completion latency. If all counters are pending or quarantined, transmission
fails with `Error::ApsCounterExhausted`.

ZCL accepts a complete `DataRequest<zb_zcl::UnsequencedFrame<Bytes>>` from its caller. The ZCL actor
consumes the unsequenced frame with its assigned transaction sequence and serializes the resulting
regular frame without changing any APS request field. ZDP constructs a complete
`DataRequest<Bytes>` with the fixed ZDO data endpoint. The APS actor owns the wrapping APS counter
allocator and submits the request plus that counter to the hardware actor.
`Aps::transmit` returns a deferred result after actor handoff, which the protocol actors forward
rather than awaiting in their command loops.

ZCL and ZDP share the same collision-safe policy within their respective actors. A communicating
request receives a transaction sequence only if its complete correlation identity is neither
pending nor quarantined. A successful correlated response releases that identity immediately.
Cancelling or timing out a response, and sending a command without tracking its response,
quarantines the identity until the corresponding late frame arrives. There is no time-based
quarantine expiry. A network-down event fails every pending response with the hardware `NoRoute`
transmission error and starts a fresh correlation epoch. Exhaustion within one correlation domain
returns `Error::TransactionSequenceExhausted` instead of replacing an older correlation.

Cancellation and timeout messages carry a coordinator-private allocation generation. The
generation never appears in a Zigbee frame; it only prevents an old lifecycle message from
removing a newer transaction after successful sequence reuse.

Each APS, ZCL, and ZDP actor consumes one bounded message inbox. Confirmation, cancellation,
timeout, network lifecycle, and ordinary request messages all use that inbox. Timeout tasks retain
only a weak sender and enqueue a timeout message after the configured duration; the actors do not
use auxiliary receivers.

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
            Event::Zcl { indication } => {
                println!(
                    "unsolicited ZCL from {:?}: {:?}",
                    indication.metadata().source(),
                    indication.asdu()
                );
            }
        }
    }
}
```

`Event::Zcl` is emitted only for inbound frames that do not match an outstanding request.
Request/response traffic is consumed by the relevant `communicate(...)` call. The event contains
the normalized `DataIndication<zb_zcl::Frame<zb_zcl::Cluster>, (), ()>` so applications retain the
received source and destination, profile and cluster identifiers, status, security information,
and link quality alongside the parsed ZCL frame.

An APS packet with cluster ID `0x0025` (`Cluster::KeepAlive`) under a supported application profile
is handled before ZCL payload decoding and produces `Device::KeepAlive`. The contained
`zb_core::destination::Device` identifies the sender by its NWK short address and APS source
endpoint. Packets whose source is not an allocated device short address or whose source endpoint is
reserved are logged and dropped instead of producing an event.

The mux forwards parsed ZCL and ZDP frames to their protocol actors inside
`DataIndication<Frame<_>, (), ()>`. It retains APS addressing, status, security mode, key index,
profile, cluster, and link quality while normalizing the backend-specific timestamp and
device-key-pair handle to `()`.

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
use zb_core::short_id::Device;
use zb_core::IeeeAddress;

async fn refresh_short_id(
    api: &impl AddressTranslation,
    ieee: IeeeAddress,
) -> Result<Device, apis_saltans_coordinator::Error> {
    api.ieee_address_to_short_id(ieee).await
}
```

Use this to consult the NCP's address table. Applications should still decide whether and how to
cache the result.

### Scanning

```rust,no_run
use apis_saltans_coordinator::{ChannelMask, FoundNetwork, ScanDuration, Scanning};

async fn scan(api: &impl Scanning) -> Result<Vec<FoundNetwork>, apis_saltans_coordinator::Error> {
    const DEFAULT_DURATION: ScanDuration =
        ScanDuration::new(5).expect("valid scan duration");

    api.scan_networks(ChannelMask::ALL, DEFAULT_DURATION).await
}
```

`scan_networks(...)` returns discovered networks. `scan_channels(...)` returns channel scan
observations. Typed channel masks and scan durations reject unsupported channel bits and the
reserved scan-duration exponent.

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
clusters, and sends bind requests for matching clusters only. Each request targets the endpoint ID
declared by that descriptor; descriptor list position has no effect.

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
use zb_aps::apsde::IndividualEndpoint;
use zb_core::{Application, Destination, Endpoint};

async fn switch_on(api: &impl OnOff) -> Result<(), apis_saltans_coordinator::Error> {
    let short_id = Device::try_from(0x1234).expect("valid short address");
    let remote_endpoint = Application::try_from(1).expect("valid remote endpoint");
    let destination =
        Destination::from(DeviceDestination::new(short_id, remote_endpoint.into()));
    let source_endpoint = IndividualEndpoint::new(Endpoint::from(
        Application::try_from(1).expect("valid local endpoint"),
    ))
    .expect("application endpoints are individual");

    api.on(destination, source_endpoint).await
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
use zb_aps::apsde::IndividualEndpoint;
use zb_core::{Application, Destination, Endpoint};
use zb_zcl::Options;

async fn set_color_temperature(
    api: &impl ColorControl,
) -> Result<(), apis_saltans_coordinator::Error> {
    let short_id = Device::try_from(0x1234).expect("valid short address");
    let remote_endpoint = Application::try_from(1).expect("valid remote endpoint");
    let destination =
        Destination::from(DeviceDestination::new(short_id, remote_endpoint.into()));
    let source_endpoint = IndividualEndpoint::new(Endpoint::from(
        Application::try_from(1).expect("valid local endpoint"),
    ))
    .expect("application endpoints are individual");

    api.move_to_color_temperature(
        destination,
        source_endpoint,
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
use zb_aps::apsde::IndividualEndpoint;
use zb_core::{Application, Endpoint};
use zb_zcl::general::basic::readable::Id as BasicReadableId;

async fn read_basic(
    api: &impl Attributes,
    short_id: Device,
) -> Result<Box<[ReadAttributeResult<BasicReadableId>]>, apis_saltans_coordinator::Error> {
    let endpoint = Application::try_from(1).expect("valid endpoint");
    let device = DeviceDestination::new(short_id, endpoint.into());
    let source_endpoint = IndividualEndpoint::new(Endpoint::from(
        Application::try_from(1).expect("valid local endpoint"),
    ))
    .expect("application endpoints are individual");

    api.read(
        device,
        source_endpoint,
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
use zb_aps::apsde::IndividualEndpoint;
use zb_core::{Application, Endpoint};
use zb_zcl::general::basic::writable::Attribute as BasicWritable;

async fn write_location(
    api: &impl Attributes,
    short_id: Device,
) -> Result<(), apis_saltans_coordinator::Error> {
    let endpoint = Application::try_from(1).expect("valid endpoint");
    let device = DeviceDestination::new(short_id, endpoint.into());
    let source_endpoint = IndividualEndpoint::new(Endpoint::from(
        Application::try_from(1).expect("valid local endpoint"),
    ))
    .expect("application endpoints are individual");
    let location = String::<16>::try_from("Living Room").expect("fits");

    let result = api
        .write(
            device,
            source_endpoint,
            [BasicWritable::LocationDescription(location)],
        )
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
response. It accepts
`zb_aps::apsde::DataRequest<zb_zcl::UnsequencedFrame<bytes::Bytes>>`. The request contains the APS
destination, profile and cluster IDs, local source endpoint, transmission options, alias, radius,
and unsequenced ZCL frame. Its await queues the frame and, for acknowledged unicast transmissions,
waits for the hardware result. The caller controls whether group and broadcast requests use APS
acknowledgements through the request's `TxOptions`. The ZCL actor consumes the unsequenced frame
with its transaction sequence number immediately before serialization.

Use `Zcl::communicate::<T>(...)` with the same request type when a typed response is expected. Its
first await queues the complete request and returns `ZclResponse<T>`. The destination must be one
16-bit network address and individual endpoint so it can be correlated. Awaiting that response
completes the APS transmission, waits for a correlated ZCL frame, and converts the received cluster
to `T` through `TryFrom`.

Use `Zdp::communicate(...)` for ZDP requests implementing `ExpectResponse<zb_zdp::Command>`. It
returns the equivalent `ZdpResponse<T::Response>`. The composed traits above are thin wrappers over
these raw transports; most of them await the protocol response internally.

## Error Model

Most APIs return `apis_saltans_coordinator::Error`:

- `Hardware(zb_hw::Error)`
- `SendError`
- `ReceiveError(RecvError)`
- `Timeout(Elapsed)`
- `InvalidResponseType(String)`
- `UnknownDevice(IeeeAddress)`
- `InvalidApplicationEndpoint(u8)`
- `InvalidZclCommunicationDestination(RequestDestination)`
- `DurationOutOfBounds(Duration)`
- `Zcl(Result<zb_zcl::Status, u8>)`
- `Zdp(Result<zb_zdp::Status, u8>)`

ZCL and ZDP status responses preserve known status enums and raw unknown status bytes.

For deferred operations, an error can occur at either await boundary. Queue, destination
validation, and actor handoff errors occur while obtaining the response future. Hardware
acceptance, APS completion, receive-channel, and conversion errors occur while awaiting that
response future.

## Runtime Configuration

Behavior is configurable through environment variables:

- `ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE`

Deferred response futures do not impose a deadline. Applications that require one can wrap the
second await with `tokio::time::timeout` and select a timeout policy appropriate to the operation.

Retry behavior for discovery or binding is intentionally not configured here anymore. Applications
that build discovery or binding workflows should apply their own retry and persistence policy.
