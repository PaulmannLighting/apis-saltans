# apis-saltans-hw Architecture

`apis-saltans-hw` is the hardware abstraction crate between coordinator logic and concrete Zigbee
network co-processor (NCP) drivers. The crate is actor-oriented: callers hold an `NcpHandle`,
send internal `Message` commands through the `Ncp` trait, and receive responses through one-shot
channels owned by each message.

## Boundaries

- `Builder` creates a backend from the coordinator endpoint descriptors and prepares support tasks.
- `Initialize` starts the command side of a prepared backend and returns an `NcpHandle`.
- `Driver` is the implementor-facing NCP command API.
- `Ncp` is the caller-facing proxy API implemented for `tokio::sync::mpsc::Sender<Message>`.
- `EventTranslator` converts backend-specific event messages into common `Event` values.
- `Datagram` carries serialized application payload bytes together with APS `Metadata`.

## Public Re-Exports

| Export | Defined in | Purpose |
| --- | --- | --- |
| `AwaitEvent` | `await_event.rs` | Convenience methods for waiting on common network events. |
| `bridge` | `bridge.rs` | Forwards and converts messages between Tokio MPSC channels. |
| `Builder` | `builder.rs` | Prepares a hardware backend and its support tasks. |
| `Datagram` | `datagram.rs` | Serialized application payload plus APS metadata. |
| `Driver` | `driver.rs` | Driver-side command API implemented by hardware backends. |
| `Error` | `error.rs` | Common crate error type. |
| `Event` | `event.rs` | Common hardware-layer event model. |
| `EventTranslator` | `event_translator.rs` | Converts backend event messages into `Event` values. |
| `FoundNetwork` | `message/found_network.rs` | Network scan result plus last-hop signal quality. |
| `Initialize` | `initialize.rs` | Starts the command side of a prepared backend. |
| `Metadata` | `datagram.rs` | APS profile and cluster metadata for a `Datagram`. |
| `Ncp` | `ncp.rs` | Caller-side API implemented for `NcpHandle`. |
| `NcpHandle` | `lib.rs` | `tokio::sync::mpsc::Sender<Message>`, the actor command handle. |
| `Network` | `message/found_network/network.rs` | Basic network information discovered during scans. |
| `PreparedHardware` | `prepared_hardware.rs` | Prepared startup bundle containing support tasks and event stream. |
| `ScannedChannel` | `message/scanned_channel.rs` | Channel scan result. |

Internal modules define additional items used by the public API but not directly exported:

| Item | Defined in | Purpose |
| --- | --- | --- |
| `Message` | `message.rs` | Internal actor command protocol between `NcpHandle` and the driver actor. |
| `SealedDriver` | `driver/sealed_driver.rs` | Blanket-implemented actor runtime for every `Driver + Send + 'static`. |

## Component Relationships

```mermaid
classDiagram
    direction LR

    class Builder {
        <<trait>>
        type HardwareEvent
        type Message
        type EventTranslator
        +new()
        +prepare()
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

    class PreparedHardware
    class Message
    class Event
    class Datagram
    class Metadata
    class FoundNetwork
    class Network
    class ScannedChannel
    class Error

    Builder --> PreparedHardware : prepares
    PreparedHardware --> Initialize : starts
    PreparedHardware --> EventTranslator : spawns
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
    participant P as PreparedHardware;
    participant T as EventTranslator;
    participant D as Driver;

    C->>B: new(endpoints);
    C->>B: prepare(hw_events);
    B-->>C: PreparedHardware;
    C->>P: start();
    P->>T: spawn translator task;
    P->>D: init();
    D-->>P: NcpHandle;
    P-->>C: NcpHandle and Event receiver;
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
semantics are represented by `apis_saltans_core::Destination`; the hardware abstraction no longer
has separate unicast, multicast, and broadcast actor messages.

## Module Inventory

```mermaid
flowchart TD
    lib["lib.rs"] --> await_event["await_event.rs"]
    lib --> bridge["bridge.rs"]
    lib --> builder["builder.rs"]
    lib --> datagram["datagram.rs"]
    lib --> driver["driver.rs"]
    driver --> sealed_driver["driver/sealed_driver.rs"]
    lib --> error["error.rs"]
    lib --> event["event.rs"]
    event --> route_error["event/route_error.rs"]
    lib --> event_translator["event_translator.rs"]
    lib --> initialize["initialize.rs"]
    lib --> message["message.rs"]
    message --> found_network["message/found_network.rs"]
    found_network --> network["message/found_network/network.rs"]
    message --> scanned_channel["message/scanned_channel.rs"]
    lib --> ncp["ncp.rs"]
    lib --> prepared_hardware["prepared_hardware.rs"]
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
device join/leave notifications, route errors, and raw received APS data as
`apis_saltans_nwk::Envelope<apis_saltans_aps::Data<bytes::Bytes>>`.

Scan commands use `FoundNetwork`, `Network`, and `ScannedChannel` to report network discovery and
channel activity results without exposing backend-specific scan response formats.

## Error Handling

`Error` is intentionally small:

- `Implementation` wraps backend-specific errors.
- `DriverSend` means the actor command channel was closed.
- `DriverRecv` means the one-shot response channel was closed.
- `NotImplemented` represents unsupported backend features.
- `NoEndpoints` represents startup without endpoint descriptors.
