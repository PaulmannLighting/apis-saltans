# apis-saltans-hw Architecture

`apis-saltans-hw` is the hardware abstraction crate between coordinator logic and concrete Zigbee
network co-processor (NCP) drivers. The crate is actor-oriented: callers hold an `NcpHandle`,
send internal `Message` commands through the `Ncp` trait, and receive responses through one-shot
channels owned by each message.

## Boundaries

- The `driver` feature exposes `Backend`, `Driver`, `EventTranslator`, `bridge`, shared
  coordinator/driver data types, command handles, common error types, and protocol crate re-export
  modules for hardware backend implementations.
- The `coordinator` feature exposes `Ncp`, `NcpHandle`, `WeakNcpHandle`, common error types, and
  coordinator-side data/event types for coordinator code.
- `Backend` defines the hardware-specific event, translator message, and translator types.
- `Driver` is the implementor-facing NCP command API.
- `Ncp` is the caller-facing proxy API implemented for `tokio::sync::mpsc::Sender<Message>`.
- `Ncp::transmit` returns the driver's one-shot completion receiver after actor handoff instead of
  awaiting the driver result inside the proxy method.
- `EventTranslator` converts backend-specific event messages into common `Event` values.
- `Datagram` carries serialized application payload bytes together with APS `Metadata`.
- `Datagram`, `Metadata`, `Event`, `FoundNetwork`, `Network`, and `ScannedChannel` are exported by
  `coordinator` and `driver`.
- `Error`, `RouteError`, `NcpHandle`, and `WeakNcpHandle` are exported by `coordinator` and
  `driver`.

## Public Re-Exports

| Export | Feature | Defined in | Purpose |
| --- | --- | --- | --- |
| `Backend` | `driver` | `driver/backend.rs` | Defines backend-specific event and translator types. |
| `bridge` | `driver` | `driver/bridge.rs` | Forwards and converts messages between Tokio MPSC channels. |
| `Clusters` | `driver` or `coordinator` | `common/clusters.rs` | Input and output cluster sets advertised by one local application endpoint. |
| `Datagram` | `driver` or `coordinator` | `common/datagram.rs` | Serialized application payload plus APS metadata. |
| `Driver` | `driver` | `driver/driver.rs` | Driver-side command API implemented by hardware backends. |
| `Error` | `driver` or `coordinator` | `common/error.rs` | Common crate error type. |
| `Event` | `driver` or `coordinator` | `common/event.rs` | Common hardware-layer event model. |
| `EventTranslator` | `driver` | `driver/event_translator.rs` | Converts backend event messages into `Event` values. |
| `FoundNetwork` | `driver` or `coordinator` | `common/message/found_network.rs` | Network scan result plus last-hop signal quality. |
| `Metadata` | `driver` or `coordinator` | `common/datagram.rs` | APS profile and cluster metadata for a `Datagram`. |
| `Ncp` | `coordinator` | `coordinator.rs` | Caller-side API implemented for `NcpHandle`. |
| `NcpHandle` | `driver` or `coordinator` | `common.rs` | `tokio::sync::mpsc::Sender<Message>`, the actor command handle. |
| `Network` | `driver` or `coordinator` | `common/message/found_network/network.rs` | Basic network information discovered during scans. |
| `RouteError` | `driver` or `coordinator` | `common/event/route_error.rs` | Route error payload used in translated hardware events. |
| `ScannedChannel` | `driver` or `coordinator` | `common/message/scanned_channel.rs` | Channel scan result. |
| `WeakNcpHandle` | `driver` or `coordinator` | `common/message.rs` | Weak sender handle for components that should not keep the actor alive. |
| `aps` | `driver` | `reexports.rs` | Re-export of `zb-aps` for driver crates. |
| `core` | `driver` | `reexports.rs` | Re-export of `zb-core` for driver crates. |
| `nwk` | `driver` | `reexports.rs` | Re-export of `zb-nwk` for driver crates. |
| `zdp` | `driver` | `reexports.rs` | Re-export of `zb-zdp` for driver crates. |

Internal modules define additional items used by the public API but not directly exported:

| Item | Defined in | Purpose |
| --- | --- | --- |
| `Message` | `common/message.rs` | Internal actor command protocol between `NcpHandle` and the driver actor. |
| `SealedDriver` | `driver/driver.rs` | Blanket-implemented actor runtime for every `Driver + Send + 'static`. |

## Component Relationships

```mermaid
classDiagram
    direction LR

    class Backend {
        <<trait>>
        type HardwareEvent
        type Message
        type EventTranslator
    }

    class Driver {
        <<trait>>
        +get_endpoints()
        +get_pan_id()
        +get_ieee_address()
        +scan_networks()
        +scan_channels()
        +allow_joins()
        +route_request()
        +short_id_to_ieee_address()
        +ieee_address_to_short_id()
        +transmit()
        +run()
        +spawn()
    }

    class SealedDriver {
        <<internal trait>>
        +run()
        +spawn()
    }

    class Ncp {
        <<trait>>
        +get_endpoints()
        +get_pan_id()
        +get_ieee_address()
        +scan_networks()
        +scan_channels()
        +allow_joins()
        +route_request()
        +short_id_to_ieee_address()
        +ieee_address_to_short_id()
        +transmit()
    }

    class NcpHandle {
        <<type alias>>
        Sender_Message
    }

    class EventTranslator {
        <<trait>>
        type Message
        +new()
        +run()
    }

    class Message
    class Clusters
    class Event
    class FullAddress
    class Datagram
    class Metadata
    class FoundNetwork
    class Network
    class ScannedChannel
    class Error

    Backend --> Driver : associated type
    Backend --> EventTranslator : associated type
    Event --> FullAddress : device membership
    EventTranslator --> Event : emits
    Driver ..> SealedDriver : run/spawn delegate
    SealedDriver --> Message : receives
    SealedDriver --> NcpHandle : creates
    NcpHandle ..|> Ncp : Sender<Message> impl
    Ncp --> Message : sends
    Message --> Clusters : endpoint response
    Message --> Datagram : carries TX payload
    Datagram --> Metadata : contains
    Message --> FoundNetwork : scan response
    Message --> ScannedChannel : scan response
    FoundNetwork --> Network : contains
```

## Driver Runtime Flow

```mermaid
sequenceDiagram
    participant I as Integration;
    participant T as EventTranslator;
    participant D as Driver;
    participant H as NcpHandle;

    I->>D: construct concrete driver;
    I->>T: construct translator with Event sender;
    I->>D: run(channel_size);
    D-->>I: NcpHandle and driver actor future;
    I->>T: run(translator inbox);
    I->>H: pass handle to coordinator startup;
```

Backend crates own concrete startup and transport wiring. The `hw` crate supplies the common driver
actor, event model, and translator traits, but does not define a separate startup feature.

## Actor Command Flow

```mermaid
sequenceDiagram
    participant C as Caller;
    participant H as NcpHandle;
    participant A as Driver actor;
    participant D as Driver;

    C->>H: scan_networks(channel_mask, duration);
    H->>A: Message::ScanNetworks;
    A->>D: scan_networks(channel_mask, duration);
    D-->>A: scan result;
    A-->>C: oneshot response;

    C->>H: transmit(destination, datagram);
    H->>A: Message::Transmit;
    H-->>C: transmission completion receiver;
    A->>D: transmit(destination, datagram);
    D-->>A: transmit result;
    A-->>C: result through completion receiver;
```

Each proxy call maps to one internal `Message` and one driver call. Destination-specific delivery
semantics are represented by `zb_core::Destination`; the hardware abstraction no longer
has separate unicast, multicast, and broadcast actor messages.

The transmit path differs from other proxy calls at the response boundary. The first await only
confirms that `Message::Transmit` entered the actor inbox and returns its one-shot receiver. The
caller decides when to await that receiver for the driver's result. This lets the coordinator wrap
hardware completion together with a later ZCL or ZDP response without blocking the command handoff.

## Module Inventory

```mermaid
flowchart TD
    lib["lib.rs"] --> common["common.rs"]
    lib --> driver["driver/mod.rs"]
    lib --> coordinator["coordinator.rs"]
    lib --> reexports["reexports.rs"]
    common --> datagram["common/datagram.rs"]
    common --> error["common/error.rs"]
    common --> event["common/event.rs"]
    event --> route_error["common/event/route_error.rs"]
    common --> message["common/message.rs"]
    message --> found_network["common/message/found_network.rs"]
    found_network --> network["common/message/found_network/network.rs"]
    message --> scanned_channel["common/message/scanned_channel.rs"]
    driver --> backend["driver/backend.rs"]
    driver --> bridge["driver/bridge.rs"]
    driver --> driver_impl["driver/driver.rs"]
    driver --> event_translator["driver/event_translator.rs"]
```

## Command Protocol

`Message` is the private actor protocol carried by `NcpHandle`. Each variant owns a one-shot
response sender so the actor can return the result of the corresponding driver call.

| `Ncp` method | `Message` variant | `Driver` method |
| --- | --- | --- |
| `get_endpoints` | `GetEndpoints` | `get_endpoints` |
| `get_pan_id` | `GetPanId` | `get_pan_id` |
| `get_ieee_address` | `GetIeeeAddress` | `get_ieee_address` |
| `scan_networks` | `ScanNetworks` | `scan_networks` |
| `scan_channels` | `ScanChannels` | `scan_channels` |
| `allow_joins` | `AllowJoins` | `allow_joins` |
| `route_request` | `RouteRequest` | `route_request` |
| `short_id_to_ieee_address` | `TranslateIeeeAddress` | `short_id_to_ieee_address` |
| `ieee_address_to_short_id` | `TranslateShortId` | `ieee_address_to_short_id` |
| `transmit` | `Transmit` | `transmit`; the proxy returns the completion receiver before the driver result is available |

## Data Model

`Clusters` is the local endpoint cluster summary returned by `get_endpoints`. The outer map is
keyed by `zb_core::Application` endpoint ID, and each `Clusters` value contains input and output
`zb_core::Cluster` sets for that endpoint.

`Datagram` is the transmit payload passed to the driver. It contains:

- `Metadata`, which identifies the APS profile and cluster.
- `bytes::Bytes`, which contains the serialized application payload.

`Event` is the receive-side model emitted by the event translator. It reports network state changes,
device join/rejoin/leave notifications carrying `zb_core::FullAddress`, route errors, and
raw received APS data as `zb_nwk::Envelope<zb_aps::Data<bytes::Bytes>>`.

Scan commands use `FoundNetwork`, `Network`, and `ScannedChannel` to report network discovery and
channel activity results without exposing backend-specific scan response formats.

## Error Handling

`Error` is intentionally small:

- `Implementation` wraps backend-specific errors.
- `DriverSend` means the actor command channel was closed.
- `DriverRecv` means the one-shot response channel was closed.
- `NotImplemented` represents unsupported backend features.
- `NoEndpoints` represents startup without endpoint cluster information.

The common and routing error enums derive `thiserror::Error`. `Implementation` retains its
backend-specific error as the source. Actor send and receive conversions remain explicit because
they intentionally reduce Tokio's channel errors to payload-free variants.
