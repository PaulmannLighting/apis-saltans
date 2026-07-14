# apis-saltans-coordinator

High-level Zigbee coordinator API built on top of [`apis-saltans-hw`](../hw).

This crate exposes a single coordinator handle (`Coordinator`) plus trait-based APIs for common coordinator operations:

- network joining control
- device state lookup/resolution
- high-level cluster commands (On/Off, Color Control)
- generic attribute read/write operations

For the internal actor graph and message routing details, see [ARCHITECTURE.md](./ARCHITECTURE.md).

## What You Get

Public API exports:

- `Coordinator`
- traits:
    - `Joining`
    - `NetworkManager`
    - `OnOff`
    - `ColorControl`
    - `Level`
    - `Attributes`
- attribute helper alias:
    - `ReadAttributeResult<T>`
- event types:
    - `Event`
    - `Zcl`
    - `EventReceiver`
- state model types:
    - `Device`, `EndpointInfo`, `DeviceAttributes`
- error type:
    - `Error`

## Coordinator Lifecycle

`Coordinator` is started with:

- an `NcpHandle` for a running hardware driver actor
- a receiver for translated hardware `Event` values
- a storage actor sender
- the local endpoint descriptors to expose on the coordinator

```rust,no_run
use apis_saltans_coordinator::Coordinator;
use tokio::sync::mpsc::Sender;
use zb_hw::{Event, NcpHandle};
use zb_zdp::SimpleDescriptor;

async fn init(
    ncp: NcpHandle,
    events: tokio::sync::mpsc::Receiver<Event>,
    storage: Sender<apis_saltans_coordinator::storage::Message>,
) -> Result<Coordinator, zb_hw::Error> {
    let endpoints: &[SimpleDescriptor] = &[/* your endpoint descriptors here */];
    Coordinator::start(ncp, events, storage, endpoints).await
}
```

To load persisted devices after startup, call `NetworkManager::load(...)` on the coordinator.

## Trait-Based API

The library intentionally uses traits to group functionality.
Import the traits you use so extension methods become available.

```rust,no_run
use apis_saltans_coordinator::{
    Attributes, ColorControl, Coordinator, Joining, Level, NetworkManager, OnOff,
};
```

### 1) Joining Control (`Joining`)

Allows opening the network for joins:

```rust,no_run
use std::time::Duration;
use apis_saltans_coordinator::Joining;

async fn allow_joins(coordinator: &impl Joining) -> Result<Duration, apis_saltans_coordinator::Error> {
    coordinator.allow_joining(Duration::from_secs(60)).await
}
```

Returns the effective duration accepted by the hardware stack.

### 2) Device/Address Resolution (`NetworkManager`)

Provides queries against coordinator-maintained network state:

- `get_ieee_address_from_short_id`
- `get_short_id_from_ieee_address`
- `get_full_address`
- `subscribe`
- `devices`

```rust,no_run
use zb_core::IeeeAddress;
use apis_saltans_coordinator::NetworkManager;

async fn resolve_example(api: &impl NetworkManager) -> Result<Option<u16>, apis_saltans_coordinator::Error> {
    let ieee = IeeeAddress::new(0, 1, 2, 3, 4, 5, 6, 7);
    api.get_short_id_from_ieee_address(ieee).await
}
```

### Event subscriptions

`NetworkManager::subscribe(channel_size)` creates a bounded Tokio channel and returns its
receiver. Every subscriber receives the coordinator events published after it subscribes; the API
does not apply per-device filtering. Dropping the receiver closes the subscription, which the
network manager removes after a delivery attempt. `channel_size` must be greater than zero.

`subscribe` returns `tokio::sync::mpsc::Receiver<Event>` directly. The separately exported
`EventReceiver` newtype wraps the same receiver type and dereferences to it, exposing methods such
as `recv()`.

`Event` currently models device lifecycle notifications and unsolicited ZCL commands:

- `Event::DeviceJoined(FullAddress)`
- `Event::DeviceLeft(FullAddress)`
- `Event::DeviceAnnounced(FullAddress)`
- `Event::Zcl(Zcl)`

For a ZCL event, `Zcl::src_address()` returns the resolved IEEE and short address,
`Zcl::src_endpoint()` returns the source endpoint or its reserved raw value, and
`Zcl::into_command()` returns the parsed `zb_zcl::Cluster` command. Incoming commands whose source
cannot be resolved are logged and are not published.

```rust,no_run
use apis_saltans_coordinator::{Event, NetworkManager};

async fn receive_events(api: &impl NetworkManager) -> Result<(), apis_saltans_coordinator::Error> {
    let mut events = api.subscribe(16).await?;

    while let Some(event) = events.recv().await {
        match event {
            Event::DeviceJoined(address) => {
                println!("device joined: {address}");
            }
            Event::DeviceLeft(address) => {
                println!("device left: {address}");
            }
            Event::DeviceAnnounced(address) => {
                println!("device announced: {address}");
            }
            Event::Zcl(event) => {
                let source = event.src_address();
                let endpoint = event.src_endpoint();
                let command = event.into_command();
                println!("ZCL command from {source} endpoint {endpoint:?}: {command:?}");
            }
        }
    }

    Ok(())
}
```

### 3) On/Off Cluster Commands (`OnOff`)

High-level helpers for standard On/Off cluster control:

- `on`
- `off`
- `toggle`

```rust,no_run
use zb_core::IeeeAddress;
use zb_core::Application;
use apis_saltans_coordinator::{Destination, OnOff};

async fn switch_on(api: &impl OnOff) -> Result<(), apis_saltans_coordinator::Error> {
    let ieee = IeeeAddress::new(0, 1, 2, 3, 4, 5, 6, 7);
    let destination = Destination::Endpoint {
        ieee_address: ieee,
        endpoint: Application::try_from(1).expect("valid endpoint"),
    };
    api.on(destination).await
}
```

### 4) Color Control Cluster Commands (`ColorControl`)

High-level color control operations:

- `move_to_xy`
- `move_to_color_temperature`

```rust,no_run
use zb_core::IeeeAddress;
use zb_zcl::Options;
use zb_core::{Application, units::{Deciseconds, Mireds}};
use apis_saltans_coordinator::{ColorControl, Destination};

async fn set_xy(api: &impl ColorControl) -> Result<(), apis_saltans_coordinator::Error> {
    let ieee = IeeeAddress::new(0, 1, 2, 3, 4, 5, 6, 7);
    let destination = Destination::Endpoint {
        ieee_address: ieee,
        endpoint: Application::try_from(1).expect("valid endpoint"),
    };
    api.move_to_xy(
        destination,
        30_000,
        30_000,
        Deciseconds::new(10).expect("valid transition time"),
        Options::empty(),
    )
    .await
}

async fn set_color_temperature(
    api: &impl ColorControl,
) -> Result<(), apis_saltans_coordinator::Error> {
    let ieee = IeeeAddress::new(0, 1, 2, 3, 4, 5, 6, 7);
    let destination = Destination::Endpoint {
        ieee_address: ieee,
        endpoint: Application::try_from(1).expect("valid endpoint"),
    };
    let color_temperature = Mireds::try_from(250).expect("valid color temperature");

    api.move_to_color_temperature(
        destination,
        color_temperature,
        Deciseconds::new(10).expect("valid transition time"),
        Options::empty(),
    )
    .await
}
```

### 5) Generic Attribute Access (`Attributes`)

The `Attributes` trait groups typed attribute reads and writes.

Use `configure_reporting(...)` to configure a node to send attribute reports. Its iterable of
reportable attribute descriptors supplies the manufacturer code, profile ID, cluster ID, attribute
IDs, and ZCL data type IDs; callers additionally provide the target device, reporting intervals,
and optional reportable-change value.

#### Reads

Use `read<T>(...)` for typed reads with a `zb_zcl::Readable` attribute ID enum.

Typed example with Basic-cluster readable IDs:

```rust,no_run
use zb_core::IeeeAddress;
use zb_zcl::general::basic::readable::Id as BasicReadableId;
use zb_core::Application;
use apis_saltans_coordinator::{Attributes, ReadAttributeResult};

async fn read_basic(
    api: &impl Attributes,
    ieee: IeeeAddress,
) -> Result<Box<[ReadAttributeResult<BasicReadableId>]>, apis_saltans_coordinator::Error> {
    api.read(
        ieee,
        Application::try_from(1).expect("valid endpoint"),
        vec![BasicReadableId::ModelIdentifier, BasicReadableId::ManufacturerName].into_boxed_slice(),
    )
    .await
}
```

#### Writes

Use `write<T>(...)` for typed writes with `zb_zcl::Writable` attributes.

Typed example with Basic writable attributes:

```rust,no_run
use zb_core::IeeeAddress;
use zb_zcl::general::basic::writable::Attribute as BasicWritable;
use zb_core::types::String;
use zb_core::Application;
use apis_saltans_coordinator::Attributes;

async fn write_location(
    api: &impl Attributes,
    ieee: IeeeAddress,
) -> Result<(), apis_saltans_coordinator::Error> {
    let location = String::<16>::try_from("Living Room").unwrap();

    let result = api
        .write(
            ieee,
            Application::try_from(1).expect("valid endpoint"),
            vec![BasicWritable::LocationDescription(location)].into_boxed_slice(),
        )
        .await?;

    // result: Vec<Result<ok_attr_id, failed_attr_id>>
    let _ = result;
    Ok(())
}
```

## Coordinator State Types

`NetworkManager::state()` returns a map of known devices keyed by IEEE address.

State model:

- `State`: persistent snapshot (`devices: Box<[Device]>`)
- `Device`:
    - `address: zb_core::Address`
    - `endpoints: BTreeMap<zb_core::Endpoint, apis_saltans_coordinator::Endpoint>`
- `Endpoint`:
    - `descriptor: zb_zdp::SimpleDescriptor`
    - `attributes: apis_saltans_coordinator::DeviceAttributes`
- `DeviceAttributes`: normalized subset of discovered Basic-cluster metadata (manufacturer/model/version/etc.)

## Error Model

All high-level API traits return `apis_saltans_coordinator::Error`:

- `Hardware(zb_hw::Error)`
- `SendError`
- `ReceiveError`
- `Timeout`
- `InvalidResponseType`
- `UnknownDevice`

This keeps trait APIs consistent while preserving underlying failure causes.

## Runtime Configuration (Environment)

Behavior is configurable via environment variables:

- `ZIGBEE_COORDINATOR_MAX_RETRIES`
- `ZIGBEE_COORDINATOR_RETRY_DELAY_SECS`
- `ZIGBEE_COORDINATOR_TASK_POOL_SIZE`
- `ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE`
