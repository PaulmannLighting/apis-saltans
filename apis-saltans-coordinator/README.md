# apis-saltans-coordinator

High-level Zigbee coordinator API built on top of [`apis-saltans-hw`](../apis-saltans-hw).

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
    - `ReadAttributes`
    - `WriteAttributes`
- attribute helper alias:
    - `ReadAttributeResult<T>`
- event types:
    - `Event`
    - `EventType`
    - `EventReceiver`
- state model types:
    - `State`, `Device`, `Endpoint`, `Attributes`
- error type:
    - `Error`

## Coordinator Lifecycle

`Coordinator` is started with:

- a hardware backend implementing `apis_saltans_hw::Start`
- the local endpoint descriptors to expose on the coordinator

```rust,no_run
use apis_saltans_coordinator::Coordinator;
use apis_saltans_zdp::SimpleDescriptor;

async fn init<T: apis_saltans_hw::Start>(hardware: T) -> Result<Coordinator, apis_saltans_hw::Error> {
    let endpoints: &[SimpleDescriptor] = &[/* your endpoint descriptors here */];
    Coordinator::start(hardware, endpoints).await
}
```

To load persisted devices after startup, call `NetworkManager::load(...)` on the coordinator.

## Trait-Based API

The library intentionally uses traits to group functionality.
Import the traits you use so extension methods become available.

```rust,no_run
use apis_saltans_coordinator::{
    ColorControl, Coordinator, Joining, Level, NetworkManager, OnOff, ReadAttributes,
    WriteAttributes,
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
- `short_id_to_address`
- `ieee_address_to_address`
- `subscribe_to_incoming_commands`
- `state` (snapshot of known devices)

```rust,no_run
use apis_saltans_core::IeeeAddress;
use apis_saltans_coordinator::NetworkManager;

async fn resolve_example(api: &impl NetworkManager) -> Result<Option<u16>, apis_saltans_coordinator::Error> {
    let ieee = IeeeAddress::new(0, 1, 2, 3, 4, 5, 6, 7);
    api.get_short_id_from_ieee_address(ieee).await
}
```

`subscribe_to_incoming_commands` returns a receiver of [`Event`] values. `Event`
contains the source address, source endpoint, and an [`EventType`]. `EventType`
is the public alias for the event payload enum and currently distinguishes
cluster-specific commands from parsed attribute reports:

- `EventType::Cluster(apis_saltans_zcl::Cluster)`
- `EventType::AttributeReport(Box<[apis_saltans_zcl::Reportable]>)`

Pass an empty device set to subscribe to all known devices, or pass IEEE
addresses to receive only matching devices.

```rust,no_run
use std::collections::BTreeSet;

use apis_saltans_coordinator::{EventType, NetworkManager};

async fn receive_events(api: &impl NetworkManager) -> Result<(), apis_saltans_coordinator::Error> {
    let mut events = api
        .subscribe_to_incoming_commands(BTreeSet::new(), 16)
        .await?;

    while let Some(event) = events.recv().await {
        match event.typ() {
            EventType::Cluster(command) => {
                let _ = command;
            }
            EventType::AttributeReport(attributes) => {
                let _ = attributes;
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
use apis_saltans_core::IeeeAddress;
use apis_saltans_core::Application;
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

High-level color control operation:

- `move_to_xy`

```rust,no_run
use apis_saltans_core::IeeeAddress;
use apis_saltans_zcl::Options;
use apis_saltans_core::{Application, units::Deciseconds};
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
        Deciseconds::from(10),
        Options::empty(),
    )
    .await
}
```

### 5) Generic Attribute Reads (`ReadAttributes`)

Two API levels are exposed:

- `read_attributes_raw(...)` for direct cluster/id reads
- `read_attributes<T>(...)` for typed reads using a `apis_saltans_zcl::ReadableAttribute` ID enum

Typed example with Basic-cluster readable IDs:

```rust,no_run
use apis_saltans_core::IeeeAddress;
use apis_saltans_zcl::general::basic::readable::Id as BasicReadableId;
use apis_saltans_core::Application;
use apis_saltans_coordinator::{ReadAttributeResult, ReadAttributes};

async fn read_basic(
    api: &impl ReadAttributes,
    ieee: IeeeAddress,
) -> Result<Box<[ReadAttributeResult<BasicReadableId>]>, apis_saltans_coordinator::Error> {
    api.read_attributes(
        ieee,
        Application::try_from(1).expect("valid endpoint"),
        vec![BasicReadableId::ModelIdentifier, BasicReadableId::ManufacturerName].into_boxed_slice(),
    )
    .await
}
```

### 6) Generic Attribute Writes (`WriteAttributes`)

Two API levels are exposed:

- `write_attributes_raw(...)` for direct record writes
- `write_attributes<T>(...)` for typed writes via `apis_saltans_zcl::WritableAttribute`

Typed example with Basic writable attributes:

```rust,no_run
use apis_saltans_core::IeeeAddress;
use apis_saltans_zcl::general::basic::writable::Attribute as BasicWritable;
use apis_saltans_core::types::String;
use apis_saltans_core::Application;
use apis_saltans_coordinator::WriteAttributes;

async fn write_location(
    api: &impl WriteAttributes,
    ieee: IeeeAddress,
) -> Result<(), apis_saltans_coordinator::Error> {
    let location = String::<16>::try_from("Living Room").unwrap();

    let result = api
        .write_attributes(
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
    - `address: apis_saltans_core::Address`
    - `endpoints: BTreeMap<apis_saltans_core::Endpoint, apis_saltans_coordinator::Endpoint>`
- `Endpoint`:
    - `descriptor: apis_saltans_zdp::SimpleDescriptor`
    - `attributes: apis_saltans_coordinator::Attributes`
- `Attributes`: normalized subset of discovered Basic-cluster metadata (manufacturer/model/version/etc.)

## Error Model

All high-level API traits return `apis_saltans_coordinator::Error`:

- `Hardware(apis_saltans_hw::Error)`
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
