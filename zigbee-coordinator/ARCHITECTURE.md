# zigbee-coordinator Architecture

This document explains how the coordinator is implemented internally, with a focus on:
- actor responsibilities
- channel topology
- end-to-end message flow
- retry/timeout and response correlation behavior

## Overview

`zigbee-coordinator` uses an actor-style runtime on top of `tokio`:
- each major subsystem runs in its own async loop (`run(...)`)
- subsystems communicate via `tokio::sync::mpsc` and `oneshot`
- long-running or per-device work is offloaded to bounded task pools (`tokio-task-pool`)

At startup, `Coordinator::start(...)` wires the full graph and returns a lightweight API handle containing key senders and the NCP handle.

## Actor Topology

```mermaid
flowchart TD
    HW[zigbee hw NCP and event stream]
    C[Coordinator handle]

    M[Mux actor]
    ZCL[ZCL transceiver actor]
    ZDP[ZDP transceiver actor]
    D[Discovery supervisor actor]
    ND[Node descriptor discovery actor]
    ED[Endpoint discovery actor]
    EDD[Endpoint descriptor discovery actor]
    AD[Attribute discovery actor]
    B[Binding actor]
    NM[Network manager actor]

    C -->|start| M
    C -->|start| ZCL
    C -->|start| ZDP
    C -->|start| D
    C -->|start| B
    C -->|start| NM

    D -->|start| ND
    D -->|start| ED
    D -->|start| EDD
    D -->|start| AD

    HW -->|events| M
    M -->|zcl frame| ZCL
    M -->|zdp frame| ZDP
    M -->|join rejoin announce| D
    M -->|device left| NM

    ND -->|node descriptor request| ZDP
    ND -->|descriptor| ED
    ED -->|active endpoint request| ZDP
    ED -->|endpoint list| EDD
    EDD -->|simple descriptor request| ZDP
    EDD -->|descriptor set| AD
    AD -->|read attributes| ZCL
    AD -->|binding::DeviceDiscovered| B
    B -->|bind request| ZDP
    B -->|new device| NM
    NM -->|rediscovery DeviceJoined| D

    ZDP -->|device announce event| D

    C -->|api call| ZCL
    C -->|state query| NM
    C -->|allow joining| HW
```

## Startup and Wiring

`Coordinator::start(...)` performs these steps:
1. Starts hardware via `Start::start(...)` and receives `(NcpHandle, Receiver<Event>)`.
2. Starts `NetworkManager` with initial persistent `State`.
3. Starts `ZCL` and `ZDP` transceivers.
4. Starts `Binding` actor with weak links to `ZDP` and `NetworkManager`, and a weak `NCP` handle.
5. Starts `Discovery` with weak links to `ZCL` and `ZDP`, and a strong sender to `Binding`.
6. Inside discovery startup, workers are wired as `NodeDescriptorDiscovery -> EndpointDiscovery -> EndpointDescriptorDiscovery -> AttributeDiscovery`, and `AttributeDiscovery` forwards completed devices directly to `Binding`.
7. Starts `Mux`, which fans out inbound hardware events.

All major actor inboxes are bounded MPSC channels (size configurable by `ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE`).

## Actor Responsibilities

### Coordinator (API facade)

Holds:
- `ncp: NcpHandle`
- sender to `ZCL` actor
- sender to `NetworkManager`

Implements user-facing traits (`OnOff`, `ColorControl`, `ReadAttributes`, `WriteAttributes`, `Joining`, `NetworkManager`) by forwarding requests to actors and composing responses.

For global attribute operations (`ReadAttributes` / `WriteAttributes`):
- the coordinator forwards raw attribute IDs produced by the ZCL attribute type (`ReadableAttribute` / `WritableAttribute`)
- no coordinator-side normalization of attribute IDs is performed
- this is required for clusters that compose IDs from a base slot plus a tagged sub-field (for example, Power Configuration battery settings where primary/secondary/tertiary battery banks use different base IDs and the setting selector is encoded in the low bits)

### Mux

Consumes raw hardware `Event`s and routes:
- `MessageReceived` with ZCL payload -> `ZCL` actor
- `MessageReceived` with ZDP payload -> `ZDP` actor
- `DeviceJoined` / `DeviceRejoined` -> `Discovery`
- `DeviceLeft` -> `NetworkManager::RemoveDevice`

### ZCL Transceiver

Responsibilities:
- send ZCL unicast/multicast through NCP
- receive inbound ZCL frames from `Mux`
- correlate response frames to pending requests

Important details:
- correlation key is ZCL sequence number (`seq`)
- pending requests are stored in `responses: BTreeMap<u8, oneshot::Sender<Cluster>>`
- `communicate(...)` sends a command and returns a oneshot-backed response future

### ZDP Transceiver

Responsibilities:
- send ZDP unicast requests through NCP (endpoint `Data`)
- receive inbound ZDP frames from `Mux`
- correlate response frames to pending requests
- handle two special inbound requests locally:
  - `MatchDescReq` -> generate and send `MatchDescRsp`
  - `DeviceAnnce` -> forward as discovery signal

Important details:
- correlation key is `(seq, response_cluster_id)`
- request cluster ID is converted with `0x8000` response mask before storing
- pending requests are stored in `responses: BTreeMap<(u8,u16), oneshot::Sender<Command>>`

### Discovery Supervisor

Receives high-level discovery triggers and feeds node descriptor discovery:
- `DeviceJoined`
- `DeviceRejoined`
- `DeviceAnnounced`

Internally owns and starts:
- `NodeDescriptorDiscovery` (`ND`)
- `EndpointDiscovery` (`ED`)
- `EndpointDescriptorDiscovery` (`EDD`)
- `AttributeDiscovery` (`AD`)

Wiring detail:
- supervisor emits discovery work only to `ND`
- `ND` forwards to `ED`, `ED` forwards to `EDD`, `EDD` forwards to `AD`
- `AD` does not route back through the supervisor; it forwards directly to `Binding`

### NodeDescriptorDiscovery (ND)

For each target device:
- sends `NodeDescReq` (via ZDP)
- retries using global retry policy (`RETRY`)
- on success forwards `(Address, NodeDescriptor)` to `EndpointDiscovery`

### EndpointDiscovery (ED)

For each target device:
- sends `ActiveEpReq` (via ZDP)
- retries using global retry policy (`RETRY`)
- on success forwards endpoint set plus node descriptor to `EndpointDescriptorDiscovery`

### EndpointDescriptorDiscovery (EDD)

For each discovered endpoint:
- sends `SimpleDescReq` (via ZDP)
- tracks per-device endpoint completion
- when all descriptors are resolved, forwards `(Endpoint -> SimpleDescriptor)` map to `AttributeDiscovery`

### AttributeDiscovery (AD)

For each application endpoint containing the Basic cluster:
- reads a fixed attribute set from `zcl::general::basic`
- converts read results into coordinator `Attributes`
- when complete, sends `binding::Message::DeviceDiscovered` directly to `Binding`

### Binding

For endpoints that advertise bindable output clusters (`OnOff`, `Level`):
- sends `BindReq` via ZDP to bind device endpoint -> coordinator IEEE + default endpoint
- tracks per-endpoint/per-cluster bind completion
- once binding is complete (or not needed), forwards `NewDevice` to `NetworkManager`

### NetworkManager

In-memory source of truth for known devices:
- `devices: BTreeMap<MacAddr8, Device>`
- `short_ids: BTreeMap<u16, MacAddr8>`

Handles:
- add/remove device updates
- short<->IEEE resolution
- full state snapshots
- rediscovery trigger: if an incoming command arrives from an unknown short ID, it resolves IEEE via NCP and sends `discovery::Message::DeviceJoined` to `Discovery`

`Subscribe` exists in message API but is currently `todo!()` in actor logic.

## Key Message Flows

## 1) Incoming hardware event routing

```mermaid
sequenceDiagram
    participant HW as NCP Event Stream
    participant M as Mux
    participant ZCL as ZCL Actor
    participant ZDP as ZDP Actor
    participant D as Discovery
    participant NM as NetworkManager

    HW->>M: Event::MessageReceived(ZCL frame)
    M->>ZCL: Message::Received

    HW->>M: Event::MessageReceived(ZDP frame)
    M->>ZDP: Message::Received

    HW->>M: Event::DeviceJoined / DeviceRejoined
    M->>D: discovery::Message

    HW->>M: Event::DeviceLeft(address)
    M->>NM: RemoveDevice(address)
```

## 2) Discovery pipeline (join -> device model)

```mermaid
sequenceDiagram
    participant M as Mux
    participant D as Discovery
    participant ND as NodeDescriptorDiscovery
    participant ED as EndpointDiscovery
    participant ZDP as ZDP Actor
    participant EDD as EndpointDescriptorDiscovery
    participant AD as AttributeDiscovery
    participant ZCL as ZCL Actor
    participant B as Binding
    participant NM as NetworkManager

    M->>D: DeviceJoined/Rejoined/Announced
    D->>ND: Discover(address)

    ND->>ZDP: communicate(NodeDescReq)
    ZDP-->>ND: NodeDescRsp(status, descriptor)
    ND->>ED: Discover{address, descriptor}

    ED->>ZDP: communicate(ActiveEpReq)
    ZDP-->>ED: ActiveEpRsp(status, endpoints)
    ED->>EDD: Discover{address, descriptor, endpoints}

    loop for each endpoint
        EDD->>ZDP: communicate(SimpleDescReq)
        ZDP-->>EDD: SimpleDescRsp
    end

    EDD->>AD: GetAttributes{address, endpoint->descriptor}

    loop for each application endpoint with Basic cluster
        AD->>ZCL: read basic attributes
        ZCL-->>AD: ReadAttributes response
    end

    AD->>B: binding::DeviceDiscovered{address, enriched endpoints}

    alt binding needed
        loop each bindable endpoint/cluster
            B->>ZDP: communicate(BindReq)
            ZDP-->>B: BindRsp(status)
        end
    end

    B->>NM: NewDevice(device)
```

## 3) API command with response (ZCL)

```mermaid
sequenceDiagram
    participant API as Coordinator API
    participant NM as NetworkManager
    participant ZCL as ZCL Actor
    participant HW as NCP
    participant M as Mux

    API->>NM: resolve IEEE -> short_id
    NM-->>API: short_id

    API->>ZCL: Message::Communicate(short_id, endpoint, payload)
    ZCL->>HW: unicast APS(ZCL, seq)

    HW->>M: Event::MessageReceived(ZCL response with seq)
    M->>ZCL: Message::Received
    ZCL-->>API: oneshot response (matched by seq)
```

## 4) API command with response (ZDP)

```mermaid
sequenceDiagram
    participant API as Internal/Discovery/Binding Caller
    participant ZDP as ZDP Actor
    participant HW as NCP
    participant M as Mux

    API->>ZDP: Message::Communicate(short_id, command)
    ZDP->>HW: unicast APS(ZDP, seq, cluster_id)
    Note right of ZDP: store pending by (seq, cluster_id|0x8000)

    HW->>M: Event::MessageReceived(ZDP response)
    M->>ZDP: Message::Received
    ZDP-->>API: oneshot response
```

## 5) Rediscovery back-channel (unknown command source)

```mermaid
sequenceDiagram
    participant ZCL as ZCL Actor
    participant NM as NetworkManager
    participant HW as NCP
    participant D as Discovery

    ZCL->>NM: Command{src_address, payload}
    alt src_address unknown in NM short-id map
        NM->>HW: short_id_to_ieee_address(src_address)
        HW-->>NM: ieee_address
        NM->>D: DeviceJoined(Address{ieee, short})
    end
```

## Concurrency Model

- Actor mailboxes are serialized per actor (single message loop).
- Potentially slow remote interactions (endpoint/descriptor/attribute/binding steps) run as separate tasks in bounded pools.
- Pools are bounded by `ZIGBEE_COORDINATOR_TASK_POOL_SIZE` to prevent unbounded fan-out.

## Reliability and Time Semantics

### Retry policy

Discovery and binding tasks use a shared retry helper:
- max attempts: `ZIGBEE_COORDINATOR_MAX_RETRIES`
- delay between retries: `ZIGBEE_COORDINATOR_RETRY_DELAY_SECS`

### Response timeouts

Handle traits apply timeouts when awaiting correlated responses:
- ZCL response timeout: `ZIGBEE_COORDINATOR_ZCL_RESPONSE_TIMEOUT_SECS`
- ZDP response timeout: `ZIGBEE_COORDINATOR_ZDP_RESPONSE_TIMEOUT_SECS`

## Channel Ownership and Lifetime

`WeakSender` is used heavily between actors/subsystems to avoid artificial lifetime extension and reference cycles:
- discovery workers hold weak links to transceivers/binding
- binding and discovery workers upgrade weak senders at use sites
- if an actor is gone, operations stop gracefully with logs instead of panics

## Data Model Progression

As a device moves through the pipeline, its representation is enriched:
1. `Address` from join/announce
2. `Address + endpoint IDs` from `ActiveEpRsp`
3. `Address + endpoint descriptors` from `SimpleDescRsp`
4. `Address + endpoint descriptors + basic attributes`
5. final `Device` inserted into `NetworkManager` (and mapped by IEEE and short ID)

## Notes and Current Gaps

- `NetworkManager::Subscribe` is declared but not yet implemented.
- Discovery/binding use best-effort async pipelines with retries and logging.
- ZDP transceiver currently includes coordinator-side handling for `MatchDescReq` and `DeviceAnnce` to support discovery and endpoint matching.
