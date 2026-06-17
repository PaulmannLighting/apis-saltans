# zigbee-coordinator

High-level Zigbee coordinator API built on top of [`zigbee-hw`](../zigbee-hw).

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
  - `ReadAttributes`
  - `WriteAttributes`
- attribute helper alias:
  - `ReadAttributeResult<T>`
- state model types:
  - `State`, `Device`, `Endpoint`, `Attributes`
- error type:
  - `Error`

## Coordinator Lifecycle

`Coordinator` is started with:
- a hardware backend implementing `zigbee_hw::Start`
- the local endpoint descriptors to expose on the coordinator
- an initial persisted `State`

```rust,no_run
use zigbee_coordinator::{Coordinator, State};
use zdp::SimpleDescriptor;

async fn init<T: zigbee_hw::Start>(hardware: T) -> Result<Coordinator, zigbee_hw::Error> {
    let endpoints: &[SimpleDescriptor] = &[];
    let state = State { devices: Box::new([]) };

    Coordinator::start(hardware, endpoints, state).await
}
```

## Trait-Based API

The library intentionally uses traits to group functionality.
Import the traits you use so extension methods become available.

```rust,no_run
use zigbee_coordinator::{
    ColorControl, Coordinator, Joining, NetworkManager, OnOff, ReadAttributes, WriteAttributes,
};
```

### 1) Joining Control (`Joining`)

Allows opening the network for joins:

```rust,no_run
use std::time::Duration;
use zigbee_coordinator::Joining;

async fn allow_joins(coordinator: &impl Joining) -> Result<Duration, zigbee_coordinator::Error> {
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
- `state` (snapshot of known devices)

```rust,no_run
use macaddr::MacAddr8;
use zigbee_coordinator::NetworkManager;

async fn resolve_example(api: &impl NetworkManager) -> Result<Option<u16>, zigbee_coordinator::Error> {
    let ieee = MacAddr8::new(0, 1, 2, 3, 4, 5, 6, 7);
    api.get_short_id_from_ieee_address(ieee).await
}
```

### 3) On/Off Cluster Commands (`OnOff`)

High-level helpers for standard On/Off cluster control:
- `on`
- `off`
- `toggle`

```rust,no_run
use macaddr::MacAddr8;
use zigbee::Endpoint;
use zigbee_coordinator::OnOff;

async fn switch_on(api: &impl OnOff) -> Result<(), zigbee_coordinator::Error> {
    let ieee = MacAddr8::new(0, 1, 2, 3, 4, 5, 6, 7);
    api.on(ieee, Endpoint::from(1)).await
}
```

### 4) Color Control Cluster Commands (`ColorControl`)

High-level color control operation:
- `move_to_xy`

```rust,no_run
use macaddr::MacAddr8;
use zcl::Options;
use zigbee::Endpoint;
use zigbee_coordinator::ColorControl;

async fn set_xy(api: &impl ColorControl) -> Result<(), zigbee_coordinator::Error> {
    let ieee = MacAddr8::new(0, 1, 2, 3, 4, 5, 6, 7);
    api.move_to_xy(
        ieee,
        Endpoint::from(1),
        30_000,
        30_000,
        10,
        Options::empty(),
    )
    .await
}
```

### 5) Generic Attribute Reads (`ReadAttributes`)

Two API levels are exposed:
- `read_attributes_raw(...)` for direct cluster/id reads
- `read_attributes<T>(...)` for typed reads using a `zcl::ReadableAttribute` ID enum

Typed example with Basic-cluster readable IDs:

```rust,no_run
use macaddr::MacAddr8;
use zcl::general::basic::readable::Id as BasicReadableId;
use zigbee::Endpoint;
use zigbee_coordinator::{ReadAttributeResult, ReadAttributes};

async fn read_basic(
    api: &impl ReadAttributes,
    ieee: MacAddr8,
) -> Result<Box<[ReadAttributeResult<BasicReadableId>]>, zigbee_coordinator::Error> {
    api.read_attributes(
        ieee,
        Endpoint::from(1),
        vec![BasicReadableId::ModelIdentifier, BasicReadableId::ManufacturerName].into_boxed_slice(),
    )
    .await
}
```

### 6) Generic Attribute Writes (`WriteAttributes`)

Two API levels are exposed:
- `write_attributes_raw(...)` for direct record writes
- `write_attributes<T>(...)` for typed writes via `zcl::WritableAttribute`

Typed example with Basic writable attributes:

```rust,no_run
use macaddr::MacAddr8;
use zcl::general::basic::writable::Attribute as BasicWritable;
use zigbee::types::String;
use zigbee::Endpoint;
use zigbee_coordinator::WriteAttributes;

async fn write_location(
    api: &impl WriteAttributes,
    ieee: MacAddr8,
) -> Result<(), zigbee_coordinator::Error> {
    let location = String::<16>::try_from("Living Room").unwrap();

    let result = api
        .write_attributes(
            ieee,
            Endpoint::from(1),
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
  - `address: zigbee::Address`
  - `endpoints: BTreeMap<zigbee::Endpoint, zigbee_coordinator::Endpoint>`
- `Endpoint`:
  - `descriptor: zdp::SimpleDescriptor`
  - `attributes: zigbee_coordinator::Attributes`
- `Attributes`: normalized subset of discovered Basic-cluster metadata (manufacturer/model/version/etc.)

## Error Model

All high-level API traits return `zigbee_coordinator::Error`:
- `Hardware(zigbee_hw::Error)`
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
- `ZIGBEE_COORDINATOR_ZCL_RESPONSE_TIMEOUT_SECS`
- `ZIGBEE_COORDINATOR_ZDP_RESPONSE_TIMEOUT_SECS`

## Optional Feature

- `smarthomelib`: enables integration with `smarthomelib::Protocol` for `Coordinator`.
