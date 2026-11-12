# apis-saltans-hw

Hardware abstraction traits and data types for Zigbee network co-processor (NCP) drivers.

This crate separates coordinator logic from concrete hardware backends. A backend implements the
`Driver` trait; coordinator code receives an `NcpHandle` and uses its inherent methods to send
commands to the driver actor. Backends own their event translation and startup wiring.

## Features

No default features are enabled. Pick the feature that matches the role of the crate that depends on
`apis-saltans-hw`.

| Feature | Intended user | Public API |
| --- | --- | --- |
| `types` | Code that only exchanges common hardware values. | Opaque handles, errors, events, and typed scan parameters and results. |
| `coordinator` | Coordinator and application code that already has a running `NcpHandle`. | Shared types plus caller-facing methods on `NcpHandle`. |
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
apis-saltans-hw = { version = "0.18", features = ["coordinator"] }
```

The command methods are available directly on `NcpHandle`:

```rust,no_run
use std::time::Duration;

use apis_saltans_hw::NcpHandle;

async fn permit_joining(ncp: &NcpHandle) -> Result<Duration, apis_saltans_hw::Error> {
    ncp.allow_joins(Duration::from_secs(60)).await
}
```

Use this feature for command-side operations such as reading the coordinator IEEE address, scanning
networks, reading local endpoint descriptors, allowing joins, resolving addresses, requesting
routes, and transmitting `zb_aps::apsde::DataRequest<bytes::Bytes>` values.

`NcpHandle::transmit(...)` hands an APS data-service request and the coordinator-assigned APS
counter to the driver actor, then waits for the backend to accept it:

```rust,ignore
ncp.transmit(request, counter).await?;
```

For frames requesting an APS acknowledgement, backends later publish
`Event::Apsde(ApsdeEvent::DataConfirm { counter, confirmation })`. The event carries the same
counter that was supplied alongside the request, while the `DataConfirm` retains the APS
destination, source endpoint, status, and backend-defined transmission timestamp.

The common `Error` type implements `std::error::Error`. Backend-specific failures retain their
source, while a stopped driver actor is represented by `Error::ActorUnavailable`.

### Hardware Events

Hardware events are grouped by their protocol responsibility:

```rust,ignore
use apis_saltans_hw::{ApsdeEvent, DeviceEvent, Event, NetworkEvent};

let network_up = Event::Network(NetworkEvent::Up);
let device_joined = Event::Device(DeviceEvent::Joined(address));
let indication = Event::Apsde(ApsdeEvent::DataIndication {
    indication,
    zdo_response_required,
});
let confirmation = Event::Apsde(ApsdeEvent::DataConfirm {
    counter,
    confirmation,
});
```

`NetworkEvent` reports network state and route errors, `DeviceEvent` reports device membership
changes, and `ApsdeEvent` reports incoming ASDUs and acknowledged transmission results. Its
timestamp type and link-key device-pair handle type are generic so each backend can retain its
native representations.

### Implementing a Driver

Enable `driver` in hardware backend crates. It exposes the common data types and protocol re-export
modules used to implement a backend:

```toml
[dependencies]
apis-saltans-hw = { version = "0.18", features = ["driver"] }
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
use apis_saltans_hw::{aps, core, zdp};
```

These modules re-export `zb-aps`, `zb-core`, and `zb-zdp` respectively. They are a convenience for
driver crates: public APIs can refer to the protocol types through
`apis_saltans_hw::core::...`, `apis_saltans_hw::aps::...`, and the other re-export modules instead
of adding direct dependencies on every protocol crate.

## Main APIs

### `Driver`

`Driver` is the implementor-facing command API. The actor runtime receives internal
`Message` values and dispatches them to the corresponding `Driver` methods.

Every driver must implement `get_endpoints()` and return one complete
`zdp::SimpleDescriptor` for each application endpoint exposed by the NCP. Descriptors include the
endpoint ID, profile ID, device ID, application version, and input/output cluster lists. The
coordinator treats this as the authoritative local endpoint set when answering ZDP match descriptor
requests and when matching clusters for bindings.

`Driver::transmit(...)` receives an `aps::apsde::DataRequest<bytes::Bytes>` and a wrapping `u8` APS
counter. Returning success means the hardware backend accepted the request. The backend uses the
request fields to perform APSDE-DATA processing and the supplied counter when constructing or
submitting the corresponding APS frame. For acknowledged transmissions, it later emits
`ApsdeEvent::DataConfirm` with that counter and a complete `aps::apsde::DataConfirm`.

Transmission uses one method:

```rust
transmit(request, counter)
```

The request contains the destination, profile, cluster, source endpoint, transmission options,
alias parameters, radius, and serialized ASDU. The separate counter is implementation correlation
metadata and is not part of the APSDE-DATA.request primitive.

Local endpoint discovery uses:

```rust
get_endpoints()
```

It returns `Box<[zdp::SimpleDescriptor]>`. The NCP must return every endpoint it advertises; callers
no longer construct or pass a separate descriptor list to the coordinator.

### `NcpHandle`

`NcpHandle` is the caller-facing proxy. Its inherent methods send commands to the driver actor
through a Tokio MPSC channel and wait for the one-shot response associated with each command.
`get_endpoints()` returns the same local simple descriptors exposed by the driver.

Every proxy method creates and awaits a response channel. `NcpHandle::transmit(...)` returns after
backend acceptance; eventual APS completion returns through the hardware event stream and is
identified by the APS counter supplied with the request. Incoming application-service data is
reported as `ApsdeEvent::DataIndication { indication, zdo_response_required }`. The `indication`
field contains an `aps::apsde::DataIndication<Bytes, T, K>`. For an incoming ZDO request, the backend
sets `zdo_response_required` from its NCP metadata to tell the coordinator whether it must generate
the response. The flag is ignored for other profiles.

### Local Endpoint Descriptors

`NcpHandle::get_endpoints()` retrieves the full simple descriptors supplied by the driver:

```rust,no_run
use apis_saltans_hw::NcpHandle;

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

### Inspecting networking failures

`Error::network_error()` returns an optional `NetworkError`, shared with the APS data-service
confirmation types. It distinguishes delivery failure, missing APS or MAC acknowledgements,
routing failures, channel access failure, congestion, exhausted local resources, network down,
not joined, address resolution failure, oversized messages, security failure, indirect-message
expiry, and full management tables. These classifications do not promise that retrying is safe.

Drivers should use `Error::network(kind, source)` for known command failures to retain the original
backend error, or `NetworkError::into()` if no backend source exists. Unclassified failures remain
`Error::backend(source)`. For asynchronous completion, use
`ConfirmStatus::Failure(error)`. Use `NetworkError::BackendStatus(u8)` when no portable
classification exists; recognized failures carry common semantics without the original vendor byte.
Use native APS statuses when the backend actually reports APS status semantics. Never interpret
`BackendStatus` without knowing the backend's status namespace.

`TransmissionError::Confirmation` retains the complete confirmation status, and its
`network_error()` accessor classifies known APS failures as well as backend-classified failures.
Local confirmation timeouts remain distinct from a reported delivery failure.

## Core network backup and restore

Network backup is optional for each driver actor. Existing `Driver` implementations and
`Driver::into_actor(capacity)` continue to work without implementing additional traits; the resulting
handle reports both capabilities disabled. Requests for an unregistered operation return
`Error::Unsupported(Operation::DumpNetwork)` or `Error::Unsupported(Operation::RestoreNetwork)`.

A capable backend implements `NetworkDump`, `NetworkRestore`, or both, and explicitly registers
those implementations at actor construction:

```rust
# #[cfg(feature = "driver")]
# {
use std::num::NonZeroUsize;

use apis_saltans_hw::{Driver, NetworkDump, NetworkRestore, NcpHandle};

fn backup_actor<D>(driver: D, capacity: NonZeroUsize)
    -> (NcpHandle, impl Future<Output = D> + Send)
where
    D: Driver + NetworkDump + NetworkRestore,
{
    driver
        .actor_builder(capacity)
        .with_network_dump()
        .with_network_restore()
        .build()
}
# }
```

The backend still owns spawning or polling the returned future. Dump and restore use the same
mailbox and actor-owned driver as ordinary commands. Disabled dispatch markers occupy no storage,
add no task or channel, and allocate nothing during ordinary commands. The complete restore request
is boxed only when restore is requested. The mailbox message remains 72 bytes on the tested
x86-64 Linux target, the same size as before backup support.

### Snapshot contents

`NetworkBackup` contains only core network state:

- Source stack and firmware identifiers and a backup schema version.
- Coordinator IEEE address, PAN ID, extended PAN ID, channel, channel mask, network update ID, and
  network security level.
- Active and additional retained/staged network keys, their sequence numbers, and outgoing security
  frame counters.
- Remote device IEEE/short addresses, child membership, individual APS/Trust Center link keys,
  link-key security counters, and incoming active-network-key counters where the stack maintains them.
- Required versioned `BackupExtension` values for remaining core network state.

Application endpoints, clusters, bindings, interviews, names, groups, scenes, reporting configuration,
and automations are outside this API. Green Power state is deferred.

An extension has a stack/vendor namespace, name, version, and opaque byte payload. Use extensions for
core state the normalized model cannot express: default Trust Center keys, link-key derivation seeds,
Trust Center/security policies, additional child records, network-key transition semantics, and replay
state for other key epochs. Every extension is required. A target must understand and restore every
extension or reject the backup before writing. Structural validity alone does not establish that a
snapshot is complete for a particular stack or transferable to another stack.

A backend must export a complete, consistent snapshot or fail. The actor's command serialization does
not stop autonomous NCP traffic: use the stack's snapshot facility or quiesce relevant operations.
Non-exportable credentials can make a backend unable to offer this capability.

### Calling the API

With `coordinator` enabled, operations are inherent methods on `NcpHandle`:

```rust,no_run
# #[cfg(feature = "coordinator")]
# {
use apis_saltans_hw::{Error, NcpHandle, NetworkBackup};

async fn capture(ncp: &NcpHandle) -> Result<Option<NetworkBackup>, Error> {
    if ncp.backup_capabilities().await?.can_dump() {
        return ncp.dump_network().await.map(Some);
    }
    Ok(None)
}
# }
```

`backup_capabilities()` describes registered operations and the API schema version. Each restore
still validates the specific backup against the actual target, including required extension versions.

To restore, pass the original snapshot and explicit `RestoreOptions::new(network_advance,
link_advance)`, with two `NonZeroU32` advances. There is no default advance. Choose advances exceeding
the traffic since capture, including any earlier restoration of the same snapshot. A finite increment
cannot make arbitrarily stale backups safe, nor recover keys or devices added after capture.

Normalized counters mean the last used counter, or a conservative upper bound on it. They are
32-bit security counters, separate from the wrapping eight-bit APS correlation counter passed to
`transmit`. Dispatch prepares a copy with checked outgoing counter advances for **all** retained
network keys and individual link keys, preserving incoming counters. Overflow or advancement to an
exhausted counter is rejected before target validation or mutation. Extension counters require
backend-specific preparation and checks. Reserve the resulting counter values before transmitting
so that the first new secured frame uses a greater value.

### Restore lifecycle and backend obligations

1. Stop coordinator protocol actors and initialize the target backend with its Zigbee network stopped.
   Backend startup must make this maintenance state available before automatically joining or forming
   a network. The old coordinator must remain offline when the replacement starts using its identity.
2. Call `NcpHandle::restore_network(original_backup, options)`. The actor validates structure and
   prepares normalized counters before calling `NetworkRestore::validate_restore`.
3. `validate_restore` checks stopped state, firmware compatibility, IEEE identity restoration, key
   import, table capacities, required extensions, and extension counter arithmetic without mutating
   persistent network state. It must reject incompatible targets before the first write.
4. `NetworkRestore::restore_network` applies the prepared snapshot. Do not advance its normalized
   counters again. Prevent external state changes between validation and writes; persist and verify
   the restored identity, credentials, parameters, and counters before constructing
   `RestoreReport::from_verified_backup`.
5. Leave the network stopped after both success and failure. Backend startup resumes the verified
   network deliberately; the generic hardware API provides no network-start command.

Direct callers of the implementor trait must validate, call `NetworkBackup::prepare_restore`, and
follow the same target validation/apply sequence themselves. Normal callers use `NcpHandle`.

Hardware restore failures should use `BackupError::restore_failed` with `RestoreFailureState::Unchanged`
or `PossiblyModified` and the backend error source. Partially changed targets must stay offline.
Backend errors and source metadata must not contain secrets. Dropping the response future does not
cancel a queued restore, but aborting the actor or losing hardware power can interrupt it. A lost
response leaves the outcome unknown and requires verification before resuming or retrying.

The result is restoration of the existing coordinator identity and network security relationships,
without manual pairing for devices covered by a compatible, sufficiently recent backup. Device
wake-up and mesh convergence still take time. Concrete backend tests must verify persistence,
identity restoration, counter behavior, failure recovery, and actual device communication.

### Serialization

The existing `serde` feature supports `NetworkBackup` and its component values. Schema version 1 is
this crate's representation, not the Open Coordinator Backup JSON format. JSON is an application
choice; the hardware library has no JSON runtime dependency. `serde_json` is used only by tests.

IEEE addresses use the existing `IeeeAddress` display representation. Extended PAN IDs are eight
bytes in display/big-endian order, channel masks are page-zero bit masks, and key material and
extension payloads serialize as byte sequences. Deserialization validates invariant-bearing records
and rejects unknown fields and unsupported schema versions; consumers should preserve the complete
snapshot, including all required extensions.

Keys and extension payloads have redacted `Debug` output, while serialization intentionally contains
secrets. Protect serialized backups at rest and in transit. Key wrappers do not promise memory
zeroization. Cloud upload, storage, encryption, scheduling, and Open Coordinator Backup conversion
belong outside the current hardware abstraction.
