# apis-saltans-hw Architecture

`apis-saltans-hw` is the hardware abstraction crate between coordinator logic and concrete Zigbee
network co-processor (NCP) drivers. The crate is actor-oriented: callers hold an `NcpHandle`,
send internal `Message` commands through the `Ncp` trait, and receive responses through one-shot
channels owned by each message.

## Boundaries

- The `driver-use` feature exposes `NcpHandle`, `Builder`, and `StartedHardware` for code that
  starts and uses a hardware backend.
- The `driver` feature includes `driver-use` and additionally exposes `Backend`, `Initialize`,
  `Driver`, `EventTranslator`, and `bridge` for hardware backend implementations.
- The `coordinator` feature exposes `Ncp` and `WeakNcpHandle` for coordinator code.
- `Backend` defines the hardware-specific event, translator message, and translator types.
- `Builder` creates a configured backend from the coordinator endpoint descriptors and prepares
  support tasks.
- `Initialize` starts the command side of a backend and returns an `NcpHandle`.
- `Driver` is the implementor-facing NCP command API.
- `Ncp` is the caller-facing proxy API implemented for `tokio::sync::mpsc::Sender<Message>`.
- `EventTranslator` converts backend-specific event messages into common `Event` values.
- `Datagram` carries serialized application payload bytes together with APS `Metadata`.
- Shared data and protocol types are exported when `driver-use`, `driver`, or `coordinator` is
  enabled.

## Public Re-Exports

| Export | Feature | Defined in | Purpose |
| --- | --- | --- | --- |
| `Backend` | `driver` | `driver/backend.rs` | Defines backend-specific event and translator types. |
| `bridge` | `driver` | `driver/bridge.rs` | Forwards and converts messages between Tokio MPSC channels. |
| `Builder` | `driver-use` | `driver_use.rs` | Constructs and starts a configured hardware backend. |
| `Datagram` | `driver-use`, `driver`, or `coordinator` | `common/datagram.rs` | Serialized application payload plus APS metadata. |
| `Driver` | `driver` | `driver/driver.rs` | Driver-side command API implemented by hardware backends. |
| `Error` | `driver-use`, `driver`, or `coordinator` | `common/error.rs` | Common crate error type. |
| `Event` | `driver-use`, `driver`, or `coordinator` | `common/event.rs` | Common hardware-layer event model. |
| `EventTranslator` | `driver` | `driver/event_translator.rs` | Converts backend event messages into `Event` values. |
| `FoundNetwork` | `driver-use`, `driver`, or `coordinator` | `common/message/found_network.rs` | Network scan result plus last-hop signal quality. |
| `Initialize` | `driver` | `driver/initialize.rs` | Starts the command side of a backend. |
| `Metadata` | `driver-use`, `driver`, or `coordinator` | `common/datagram.rs` | APS profile and cluster metadata for a `Datagram`. |
| `Ncp` | `coordinator` | `coordinator.rs` | Caller-side API implemented for `NcpHandle`. |
| `NcpHandle` | `driver-use`, `driver`, or `coordinator` | `common.rs` | `tokio::sync::mpsc::Sender<Message>`, the actor command handle. |
| `Network` | `driver-use`, `driver`, or `coordinator` | `common/message/found_network/network.rs` | Basic network information discovered during scans. |
| `StartedHardware` | `driver-use` | `driver_use.rs` | Started hardware support tasks and public handles. |
| `ScannedChannel` | `driver-use`, `driver`, or `coordinator` | `common/message/scanned_channel.rs` | Channel scan result. |
| `WeakNcpHandle` | `coordinator` | `coordinator.rs` | Weak sender handle for components that should not keep the actor alive. |

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

    class Builder {
        <<trait>>
        +new()
        +start()
    }

    class Initialize {
        <<trait>>
        +init()
    }

    class Driver {
        <<trait>>
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
        +get_pan_id()
        +get_ieee_address()
        +get_address()
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

    class StartedHardware
    class Message
    class Event
    class FullAddress
    class Datagram
    class Metadata
    class FoundNetwork
    class Network
    class ScannedChannel
    class Error

    Builder ..|> Backend : requires
    Builder --> StartedHardware : returns
    Builder --> Initialize : starts
    Builder --> EventTranslator : creates translator
    StartedHardware --> NcpHandle : contains
    StartedHardware --> Event : receives
    Event --> FullAddress : device membership
    Initialize --> NcpHandle : returns
    EventTranslator --> Event : emits
    Driver ..> SealedDriver : run/spawn delegate
    SealedDriver --> Message : receives
    SealedDriver --> NcpHandle : creates
    NcpHandle ..|> Ncp : Sender<Message> impl
    Ncp --> Message : sends
    Message --> Datagram : carries TX payload
    Datagram --> Metadata : contains
    Message --> FoundNetwork : scan response
    Message --> ScannedChannel : scan response
    FoundNetwork --> Network : contains
```

## Startup Flow

```mermaid
sequenceDiagram
    participant C as Coordinator;
    participant B as Builder;
    participant S as StartedHardware;
    participant T as EventTranslator;
    participant D as Driver;

    C->>B: new(endpoints);
    C->>B: start(hw_events);
    B->>T: create translator future;
    B->>D: init();
    D-->>B: NcpHandle;
    B-->>C: StartedHardware;
    C->>S: poll bridge and translator futures;
```

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
    A->>D: transmit(destination, datagram);
    D-->>A: transmit result;
    A-->>C: oneshot response;
```

Each proxy call maps to one internal `Message` and one driver call. Destination-specific delivery
semantics are represented by `zb_core::Destination`; the hardware abstraction no longer
has separate unicast, multicast, and broadcast actor messages.

## Module Inventory

```mermaid
flowchart TD
    lib["lib.rs"] --> common["common.rs"]
    lib --> driver["driver/mod.rs"]
    lib --> driver_use["driver_use.rs"]
    lib --> coordinator["coordinator.rs"]
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
    driver --> initialize["driver/initialize.rs"]
```

## Command Protocol

`Message` is the private actor protocol carried by `NcpHandle`. Each variant owns a one-shot
response sender so the actor can return the result of the corresponding driver call.

| `Ncp` method | `Message` variant | `Driver` method |
| --- | --- | --- |
| `get_pan_id` | `GetPanId` | `get_pan_id` |
| `get_ieee_address` | `GetIeeeAddress` | `get_ieee_address` |
| `scan_networks` | `ScanNetworks` | `scan_networks` |
| `scan_channels` | `ScanChannels` | `scan_channels` |
| `allow_joins` | `AllowJoins` | `allow_joins` |
| `route_request` | `RouteRequest` | `route_request` |
| `short_id_to_ieee_address` | `TranslateIeeeAddress` | `short_id_to_ieee_address` |
| `ieee_address_to_short_id` | `TranslateShortId` | `ieee_address_to_short_id` |
| `transmit` | `Transmit` | `transmit` |

## Data Model

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
- `NoEndpoints` represents startup without endpoint descriptors.
