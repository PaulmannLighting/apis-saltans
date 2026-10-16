# apis-saltans-aps Architecture

`apis-saltans-aps` models Zigbee APS frames, the APS data-service boundary, and
stateful defragmentation for raw APS data payloads.

```mermaid
flowchart TD
    Indication["APSDE DataIndication&lt;Data&lt;Bytes&gt;, T, K&gt;"]
    Assembler["frame::data::defragmentation::Assembler"]
    Index["Index<br/>Source + APS counter"]
    Transaction["Transaction<br/>header + fragment slots"]
    Frame["Reassembled Data&lt;Bytes&gt;"]

    Indication --> Assembler
    Assembler --> Index
    Index --> Transaction
    Transaction --> Frame
```

## Frame Modules

| Module                   | Responsibility                                                |
|--------------------------|---------------------------------------------------------------|
| `frame::control`         | APS frame-control bitfields and delivery mode decoding.       |
| `frame::data`            | APS data headers and payload-carrying frame types.            |
| `frame::command`         | APS command frame/header structures.                          |
| `frame::acknowledgement` | APS acknowledgement frame structures.                         |
| `frame::extended`        | Extended APS header fields, including fragmentation metadata. |
| `broadcast`              | Well-known Zigbee broadcast addresses.                        |
| `apsde`                  | APSDE request, confirmation, indication, and support types.   |

APS data headers retain cluster and profile identifiers as their raw wire values. Their
`cluster()` and `profile()` accessors provide typed `zb_core::Cluster` and `zb_core::Profile`
interpretations when the identifiers are known, while the raw accessors remain available for
unknown or manufacturer-specific values.

## APS Data Entity

The `apsde` module models the service-access-point boundary between a Zigbee
next-higher-layer entity and the APS data entity. It contains value types only;
actors, queues, persistence, and security processing belong to implementations
using the crate.

```mermaid
flowchart LR
    NHLE["Next-higher-layer entity"]
    Request["DataRequest&lt;A&gt;"]
    APSDE["APS data entity implementation"]
    Confirm["DataConfirm&lt;T&gt;"]
    Indication["DataIndication&lt;A, T, K&gt;"]
    Frames["APS wire frames"]

    NHLE --> Request
    Request --> APSDE
    APSDE --> Confirm
    Confirm --> NHLE
    APSDE --> Indication
    Indication --> NHLE
    APSDE --> Frames
```

Primitive-specific destination and source enums replace a loose address mode
plus optional fields. Each enum exposes only the modes legal in its context.
Group requests include their required NWK broadcast selector.
Network broadcasts retain their receiver-set address and endpoint explicitly.
Received NWK broadcasts likewise retain their broadcast receiver set and endpoint, allowing
higher layers to distinguish broadcast requests from unicast requests addressed to the local
device.
`IndividualEndpoint` excludes the APS broadcast endpoint, while request and
confirmation destinations use `zb_core::Endpoint` where the specification
permits it.
`NetworkDestination` is the narrower address-and-endpoint pair used by APIs
that require an individually addressed NWK peer.

`DataIndication::map_context` allows a consumer to normalize the implementation-defined timestamp
and link-key device-pair handle without rebuilding or discarding the remaining indication
metadata.

ASDU length is derived from byte-like payloads, preventing disagreement
between an explicit length and the ASDU. Generic parameters preserve
implementation-defined timestamp and link-key device-pair handle
representations. Unclassified backend failures and security-processing statuses retain
raw 8-bit codes so unknown statuses are not discarded. Recognized confirmation failures
carry common networking semantics.

`Security<K>` encodes conditional fields as variants, ensuring a key index and
key-pair handle exist only for link-key-secured ASDUs.

## Defragmentation

`frame::data::defragmentation::Assembler` owns a map of in-progress
transactions. Each
transaction is keyed by:

- `apsde::Source`, because APS counters are source-scoped;
- APS frame counter, because fragments of one APS frame share the counter.

The first fragment stores the original APS data header and opens the payload
slot vector. Follow-up fragments are inserted by block number. When every slot
is filled, the transaction concatenates all payload fragments, drops the
extended header from the saved data header, and returns a rebuilt
`Data<bytes::Bytes>`.

Invalid fragmentation states are intentionally drop-only:

- a frame marked as both first and follow-up fragment is rejected;
- fragmented frames without a block number are rejected;
- first fragments with total block count `0` are rejected;
- follow-up fragments without an existing transaction are rejected;
- out-of-bounds follow-up fragments drop the transaction.

## Primitive Enum Conversions

Fieldless enums with an integer `#[repr]`, such as APS broadcast addresses, frame types, and
delivery modes, derive `num_enum::IntoPrimitive` and `num_enum::TryFromPrimitive`. Repr enums with
payload-carrying variants cannot use these derives and retain their representation-specific parsing.

## Named Construction Parameters

`DataRequest::new` accepts `RequestParameters` and a payload. Required addressing and protocol
identifiers are named fields; optional transmission options, alias, and radius retain their
existing setters and defaults. `IndicationMetadata` is itself a named parameter record, so it
can be built directly without a duplicate options type. Its address and security field types
continue to enforce their existing invariants.

Data frame header constructors accept `DataHeaderParameters` and `UnicastHeaderParameters`.
The constructors derive control flags and encode endpoint values; callers cannot supply a
control field inconsistent with the named constructor options. Parsing and wire layout are
unchanged.

## Networking failure classifications

`apis-saltans-core::NetworkError` carries portable failure semantics, including the acknowledgement layer where
known. `ConfirmStatus::Failure` carries this error directly. Unclassified bytes are retained in
`NetworkError::BackendStatus`; recognized failures do not retain a redundant vendor code. Native
APS failures are classified by `ConfirmStatus::network_error()`, which also returns unclassified
backend failures without interpreting their bytes.

## Allocation boundary

The crate root is unconditionally `no_std`. The optional, default-disabled `alloc` feature selects
`bytes::Bytes` as `Payload`, `Vec<Option<Bytes>>` for transaction slots, and `BTreeMap` for pending
transactions. Bytes disables its `std` default. Other workspace consumers opt into `alloc` explicitly.

Without `alloc`, `payload/bounded.rs` supplies a heapless owned payload with unprefixed wire codecs.
Transaction slots use `heapless::Vec<Option<heapless::Vec<u8, FRAGMENT_CAPACITY>>, FRAGMENTS_CAPACITY>`;
the assembler uses `heapless::LinearMap<Index, Transaction, TRANSACTIONS_CAPACITY>`. Map order is not
observable in reassembly. Fragmentation retains the original payload and copies slices using an
offset instead of shifting the remaining bytes on each iteration.

```mermaid
flowchart LR
    Env[Compile-time environment] --> Limits[capacity.rs]
    Limits --> Payload[Bounded payload]
    Limits --> Slots[Bounded fragment slots]
    Limits --> Map[Bounded transaction map]
    Payload --> Fragments[Fragmentation iterator]
    Slots --> Map
    Map --> Assembler[Assembler]
    Assembler --> Payload
```

`build.rs` tracks each environment variable so stable `const_env` values are refreshed on rebuild.
Compile-time assertions enforce positive limits, fragment size no greater than payload size, and
fragment counts representable by the protocol. No heapless length-prefix codec is used for raw APS
payloads. Construction and decoding reject capacity overflow instead of dropping excess bytes.

`try_add` reports errors independently of completion. It validates new transactions before insertion;
map exhaustion never evicts another key. Follow-up processing removes its transaction, validates and
replaces the indexed block, and either returns a complete frame or reinserts the transaction. Failed
follow-ups discard that transaction. The total-size calculation excludes the replaced slot, so a
duplicate fragment does not consume extra capacity. `add` preserves the old completion-only API.
