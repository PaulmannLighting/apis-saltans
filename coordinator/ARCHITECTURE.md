# apis-saltans-coordinator Architecture

The coordinator is a transport and protocol-helper layer built around bounded Tokio actors.
Applications own device registries, discovery policy, retries, binding selection, and persistence.

## Actor Topology

```mermaid
flowchart TD
    HW[Hardware driver actor and event stream]
    C[Coordinator handle]
    APS[APS actor]
    ZCL[ZCL actor]
    ZDP[ZDP actor]
    ZDPO[Bounded ZDP server operations]
    OTA[OTA server]
    OTAA[OTA API forwarder]
    OTAS[OTA subscription forwarder]
    M[Mux task]
    APP[Application event receiver]

    C -->|ZCL API| ZCL
    C -->|ZDP API| ZDP
    C -->|schedule update| OTAA
    OTAA -->|ServerEvent::Message| OTA
    C -->|NCP helper APIs| HW
    ZCL -->|Data&lt;Bytes&gt;| APS
    ZDP -->|Data&lt;Bytes&gt;| APS
    ZCL -->|endpoint descriptor query| HW
    ZDP -->|spawn received request| ZDPO
    ZDPO -->|endpoint and address queries| HW
    ZDPO -->|response frame| APS
    ZDPO -->|completion through inbox| ZDP
    APS -->|NcpHandle::transmit| HW
    HW -->|zb_hw::Event| M
    M -->|received ZCL frame| ZCL
    M -->|received ZDP frame| ZDP
    M -->|network and device events| APP
    ZCL -->|unmatched ZCL frame| APP
    ZCL -.->|lazy filtered weak subscription| OTAS
    OTAS -->|Message::Received through weak sender| OTA
    OTA -->|commands and replies| ZCL
    ZDP -->|device announcements| APP
```

`Coordinator::start` creates the APS, ZCL, ZDP, and OTA actors plus the event mux. Actor inboxes
use `ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE`.

The `event.rs` façade owns application-visible event types and re-exports its internal
`event::sink::EventSink`. The sink centralizes non-blocking application-channel delivery for the
mux and protocol actors without exposing that delivery mechanism through the crate's public API.

ZDP commands enter their actor as complete `DataRequest<Bytes>` values because their local source
endpoint is always the ZDO data endpoint. ZCL commands enter their actor as complete
`DataRequest<UnsequencedFrame<Bytes>>` values; the actor assigns the ZCL transaction sequence,
serializes the resulting regular frame, and preserves all APS fields.

## APS Actor

The APS transceiver stores a concrete `zb_hw::NcpHandle`; it is the only coordinator actor that
transmits directly through that handle. It also owns the wrapping eight-bit Zigbee APS counter
allocator.
For every outgoing message it:

1. consumes the supplied `zb_aps::apsde::DataRequest<bytes::Bytes>`
2. assigns a counter that is neither pending nor quarantined
3. reserves the counter and spawns a background operation that forwards the request to the
   hardware actor
4. receives backend acceptance or rejection from that operation through its ordinary inbox
5. resolves an unacknowledged caller after hardware acceptance, or retains an acknowledged caller
   under the counter
6. quarantines counters released by dropped callers or network loss until the corresponding late
   confirmation arrives or its confirmation deadline makes the actor terminal

Its command protocol contains:

```text
Transmit {
    request: zb_aps::apsde::DataRequest<bytes::Bytes>,
    response: oneshot::Sender<Result<TransmissionResponse, crate::Error>>,
}
SubmissionFinished { token, result }
Confirm { counter, status }
Cancel { token }
ConfirmationTimeout { token }
NetworkDown
HardwareUnavailable
```

The `Aps` handle wraps the APS actor's `Sender<Message>`. Its inherent `transmit` method queues the
actor message and returns a deferred `TransmissionResponse`. It creates a completion channel for
every transmission. The APS actor resolves that channel when the hardware rejects or accepts an
unacknowledged request. When its options contain `TxOptions::ACKNOWLEDGED_TRANSMISSION` and its
destination is an individual network or extended address, hardware acceptance instead stores the
sender until the APS result arrives. Group and broadcast transmissions never await APS
acknowledgements. Its completion method forwards hardware APSDE confirmations from the mux.
The actor never awaits backend acceptance in its message loop. Each submission runs in a spawned
operation and posts `SubmissionFinished` back into the actor's one bounded inbox. This keeps the
actor available for confirmations, cancellation, and network or hardware lifecycle messages while
the NCP is slow. The counter remains reserved during submission. A confirmation that overtakes the
submission-completion message is buffered until acceptance is known. Cancellation or network loss
during submission drops or fails the caller but retains the reservation until rejection makes
reuse safe or acceptance requires quarantine.

Cancellation and confirmation-timeout events use the same inbox; timeout tasks hold a weak sender,
sleep, and enqueue `ConfirmationTimeout`. Submission-completion, cancellation, and timeout
messages carry a coordinator-private allocation generation in addition to the counter. That
generation never crosses the hardware boundary and prevents an old lifecycle message from
affecting a newer transmission after successful counter reuse.

- Acknowledged unicast frame: retain the caller's response sender under the APS counter and await
  `ApsdeEvent::DataConfirm`.
- Unacknowledged, group, or broadcast frame: resolve the caller response after backend acceptance.

The allocator scans all 256 counter values and never reuses one while it is pending or quarantined.
Cancellation and network loss quarantine an accepted acknowledged transmission's counter until a
late confirmation for that counter arrives. The quarantine has no clock-based expiry, because the
hardware API makes no promise about maximum confirmation latency or a reset boundary after which
old confirmations cannot arrive. If the confirmation is still missing at its deadline, the APS
actor stops and requires coordinator reconstruction instead of ever reusing the counter. Timeout
messages retain the allocation generation, including while a counter is quarantined, so a stale
timeout cannot stop the actor for a newer allocation.

`Error::ApsCounterExhausted` can still report that all counters are concurrently pending or
quarantined before their deadlines. Successful or failed confirmations release their counters
immediately. Unacknowledged transmissions release their reservation after backend acceptance
without awaiting a confirmation, and rejected transmissions never enter the
awaiting-confirmation phase.

```mermaid
sequenceDiagram
    participant P as ZCL or ZDP actor
    participant A as APS actor
    participant O as Submission operation
    participant H as Hardware actor
    participant M as Event mux

    P->>P: serialize protocol payload
    P->>P: construct DataRequest&lt;Bytes&gt;
    P->>A: Transmit request
    A-->>P: deferred transmission response
    A->>A: assign APS counter
    A->>O: spawn transmit request and counter
    O->>H: transmit request and counter
    H-->>O: accepted
    O-->>A: SubmissionFinished
    alt unacknowledged transmission
        A-->>P: resolve deferred APS result
    else acknowledged transmission
        A->>A: store response under counter
        H-->>M: Event::Apsde with counter and DataConfirm
        M-->>A: counter and confirmation status
        A-->>P: resolve deferred APS result
    end
```

## ZCL Actor

The ZCL actor:

- owns a collision-safe ZCL transaction-sequence allocator
- receives parsed ZCL frames as normalized `DataIndication<Frame<Cluster>, (), ()>` values
- accepts complete `DataRequest<UnsequencedFrame<Bytes>>` values
- consumes each unsequenced frame with the assigned transaction sequence and serializes the
  resulting `Frame<Bytes>` while preserving every APS request field
- sends the resulting `DataRequest<Bytes>` through the APS actor
- stores response correlation channels for `communicate`
- registers generic filtered subscriptions received through its actor inbox
- unregisters subscriptions by channel identity and prunes subscriptions whose receivers have closed
- correlates responses before delivering unmatched frames to generic internal subscriptions
- sends replies with an explicitly supplied ZCL transaction sequence
- routes unmatched received commands to the application event channel

For response-free `transmit` messages and replies, the actor forwards the deferred APS result to
the caller. An individual `transmit` must disable ZCL Default Responses; its sequence allocator
skips pending and quarantined identities but does not retain the selected identity. For
`communicate`, including the public `communicate_default` helper, the actor inserts the correlation
entry before transmitting and returns an `ApsProtocolResponse` containing both the deferred APS
result and protocol receiver. The actor therefore continues processing commands while
acknowledgements are pending. Awaiting the internal response completes APS transmission before
polling the correlated protocol response. Reply transmission preserves the request transaction
sequence instead of allocating a new one.

The allocator scans the complete 256-value sequence space for the request's correlation domain and
never replaces a pending entry. Successful responses release their correlation identity
immediately. Response-free transmissions do not enter protocol quarantine. The actor expires
pending protocol responses after 30 seconds. Timed-out and cancelled tracked responses remain
quarantined until a late frame arrives or a further 30-second grace period expires. Dropping an
`ApsProtocolResponse` enqueues `Cancel` through the actor's ordinary bounded inbox. Per-response
and quarantine timer tasks hold a weak sender and enqueue `ResponseTimeout` or
`QuarantineTimeout` through that same inbox. These messages carry a coordinator-private allocation
generation so a stale timer cannot remove a reused transaction. The actor has no auxiliary
receiver and processes its inbox with one `recv` loop. A network-down message fails pending
responses and begins a fresh correlation epoch.

Source-endpoint policy belongs to the caller. The ZCL actor does not query or cache local endpoint
descriptors. High-level cluster helpers therefore require an explicit `IndividualEndpoint`, while
the raw `Zcl` API accepts the complete `DataRequest`. A communicating request must use a 16-bit
network destination with one individual remote endpoint because response correlation requires both
values.

## OTA Upgrade Server

The OTA subsystem installs a ZCL subscription for cluster-specific, client-to-server OTA Upgrade
frames on demand. When the OTA actor admits its first device update, it sends the subscription
through the ZCL actor handle before spawning the destination transfer and its Image Notify
operation. The channel ordering therefore registers the subscription before the client can respond
to the offer. A lightweight forwarding task converts subscribed frames into ordinary OTA
`Message::Received` values. Concurrent updates and replacements reuse the same subscription and
forwarding task. When the last destination transfer finishes, the OTA server aborts the forwarding
task and sends an explicit unsubscribe message to ZCL. A later update batch registers a new
subscription.

ZCL applies only the subscription's typed cluster, scope, and direction filter; the OTA forwarding
task performs the typed `Cluster::OtaUpgrade` match. Response correlation includes the direction
opposite the outgoing request and runs before subscription delivery. An unrelated client request
therefore remains available to the subscription, while a matching response cannot be consumed by
it. ZCL uses non-blocking delivery for each bounded subscription channel. A full channel retains
its subscription but sends the current frame through application-event routing. A closed channel
removes the subscription immediately. Subscription setup is no longer part of coordinator startup
or constructor wiring. Subscription messages contain the complete normalized `DataIndication`;
the OTA forwarder narrows only its parsed ZCL command payload and preserves the original metadata.

The OTA server awaits one private event inbox. A small API task forwards public `Message` values
into it, subscription forwarding tasks send received frames into it, and destination supervisors
send transfer completion into it. When the public OTA channel closes, the API task queues a shutdown
event. The server therefore does not manually poll multiple receivers or its transfer tasks.

Each public update future owns one cancellation sender. Dropping or explicitly cancelling the
future resolves a receiver owned by a weak forwarding task, which sends a generation-tagged
`Cancel` message through the destination transfer's ordinary inbox. Discovery,
block-inactivity, and total-transfer timer tasks use the same inbox. Replacements abort the
previous generation's lifecycle tasks and start new deadlines; generation checks make already
queued stale messages harmless. Discovery ends only after a compatible query or valid data
request, block activity resets its deadline, and the total-transfer deadline never resets.

Each update carries a `FullAddress` plus its remote endpoint. The transfer pins the IEEE identity
and current NWK short address for Image Notify transmission and destination-restricted image
checks. Before the server routes any inbound OTA request, it resolves the request's NWK source
through the NCP and compares the result with the pinned IEEE address. A stale offer cannot therefore
be inherited by a different device after NWK short-address reuse. Query Specific File requests,
optional request-node addresses, and OTA image header destinations are validated against that same
identity.

The API task owns the sole strong sender for the private event inbox. Subscription and transfer
forwarders hold only weak senders. ZCL and those forwarders therefore cannot keep the OTA actor
alive. When the external OTA senders are dropped, the API task queues shutdown, the OTA actor exits,
and it aborts the subscription and active transfer tasks. This avoids a strong
ZCL-to-OTA-to-ZCL actor cycle.

Normal OTA commands and replies retain the default acknowledged APS transmission option. Image Page
block responses use empty `TxOptions`; their deferred APS result therefore completes after hardware
backend acceptance instead of waiting for an acknowledgement. The page task applies the requested
spacing and advances the ZCL transaction sequence between blocks.

## ZDP Actor

The ZDP actor:

- owns the cloneable context used for NCP queries and APS replies
- receives parsed ZDP frames as normalized `DataIndication<Frame<Command>, (), ()>` values
- owns a collision-safe ZDP transaction-sequence allocator
- uses profile `0x0000` and endpoint `0x00`
- sends APS metadata and serialized ZDP frames through the APS actor
- correlates request and response commands
- dispatches incoming requests to bounded background operations for endpoint and address queries
- handles device announcements and incoming address, descriptor, endpoint, match-descriptor,
  system-server-discovery, and permit-joining requests

Received response commands and device announcements are handled synchronously in the actor.
Requests that can query the NCP or enqueue an APS reply run in tracked background operations, with
at most `ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE` operations active at once. Each normal completion is
returned through the actor's ordinary inbox. Network-down, hardware-unavailable, actor-inbox
closure, and actor shutdown abort all active request-serving operations. If the operation limit is
already occupied, the new request is logged and dropped rather than allowing work to grow without
bound.

Outgoing ZDP communication similarly reserves its correlation identity synchronously and performs
the possibly backpressured APS actor handoff in a bounded background submission. The actor retains
the submission's caller channel, protocol receiver, correlation token, and abort handle until the
handoff completion returns through its inbox. This keeps response correlation, cancellation,
timeout, and lifecycle messages moving through the ZDP actor while another actor is congested.

At a network-down boundary, the actor aborts all unfinished handoffs and fails their callers
immediately. A handoff may already have exposed its encoded transaction sequence to APS before its
completion message is processed, so the actor preserves those wire identities in protocol
quarantine while resetting the rest of the response epoch. Each preserved identity follows the
normal quarantine timeout or is released by its late response. A stale handoff-completion message
therefore cannot return an old response object or let an old response complete a new request.

For local `Active_EP_req` and `Simple_Desc_req` commands, the actor uses the NCP's current endpoint
descriptors. Single-device address requests use the NCP's address-translation operations.
Extended address requests return `NOT_SUPPORTED` because the hardware abstraction does not expose
an associated-device list. `Power_Desc_req` returns `NO_DESCRIPTOR` because the coordinator startup
configuration does not contain a power descriptor. `System_Server_Discovery_req` receives a
response only when the requested mask intersects the server mask advertised in the coordinator's
node descriptor. The actor reads broadcast delivery directly from the normalized APSDE destination
metadata. It suppresses an empty `Match_Desc_rsp` for a broadcast `Match_Desc_req` and never sends a
`Mgmt_Permit_Joining_rsp` for a broadcast request. It rejects every unicast
`Mgmt_Permit_Joining_req` with `INVALID_REQUESTTYPE`; only the local `Joining` API controls the
hardware joining state.

ZDP responses generated locally also travel through the APS actor, so their APS counters and
acknowledgement behavior follow the same path as outgoing requests. A weak completion task retains
each deferred APS result and sends failures through the ZDP actor's ordinary inbox. Backend
rejection and unsuccessful acknowledged completion are therefore observed without blocking the
actor while the hardware result is pending.

## Response Correlation

Pending ZCL and ZDP requests are keyed by an internal correlation `Key` containing:

- remote short address
- endpoint
- cluster ID
- profile ID
- optional ZCL manufacturer code
- protocol transaction sequence

The `correlation.rs` façade exposes the correlation types and timeout policy. Its `key`,
`lifecycle`, and `registry` submodules respectively own protocol identity construction,
cancellation tokens, and actor-owned response state.

Received ZDP indications can produce a correlation key only when both their source and destination
use endpoint `0x00`. This validation is repeated at the ZDP actor boundary so malformed
profile-zero traffic cannot complete a pending exchange even if it bypasses normal mux parsing.

The mux parses successful APSDE data indications and forwards them to the appropriate protocol
actor. Each actor derives the key directly from the received indication metadata and parsed frame
and removes the matching one-shot sender. Each protocol actor permits up to 256 unavailable
identities within one correlation domain. It returns `TransactionSequenceExhausted` when no
sequence is available and expires pending responses after the compile-time
`ZIGBEE_COORDINATOR_PROTOCOL_RESPONSE_TIMEOUT_SECS` interval. Response-free ZCL transmissions skip
unavailable identities without reserving the selected sequence. Cancelled and timed-out tracked
identities remain quarantined until a late frame arrives, the compile-time
`ZIGBEE_COORDINATOR_PROTOCOL_QUARANTINE_TIMEOUT_SECS` grace period expires, or the network goes
down. Both intervals default to 30 seconds and must be greater than zero.

APS, ZCL, and ZDP each own exactly one bounded message receiver. Hardware events, API requests,
cancellations, response and quarantine timeout notifications, and network lifecycle notifications
are serialized through that actor's single inbox. The mux uses non-blocking delivery for received
ZDP frames. If the ZDP inbox is full, it logs and drops the frame so protocol overload cannot delay
APS confirmations or other hardware events. The request's normal response timeout reports a
missing dropped response to a waiting caller.

```mermaid
sequenceDiagram
    participant API as Caller
    participant P as Protocol actor
    participant A as APS actor
    participant H as Hardware
    participant M as Mux
    participant R as Protocol response

    API->>P: communicate
    P->>P: allocate sequence and store correlation
    P->>A: acknowledged APS frame
    A->>H: frame with assigned APS counter
    H-->>A: accepted
    H-->>M: Event::Apsde with counter and DataConfirm
    M-->>A: counter and confirmation status
    A-->>P: correlated APS result
    P-->>API: protocol response future
    API->>R: await
    H->>M: DataIndication with received ASDU
    M->>P: parsed protocol frame
    P->>P: match and remove correlation
    P-->>R: raw response
    R-->>API: converted typed response
```

`CommunicationResponse<Raw, T>` contains the deferred APS completion and the correlated protocol
receiver. Polling it completes APS transmission first, then waits for the correlated command and
applies `TryFrom`.

## Mux and Events

The mux consumes generic `zb_hw::Event<T, K>` values. It forwards network and device lifecycle
events to the application, accepts successful `DataIndication<Bytes, T, K>` values, parses
network-profile ASDUs as ZDP, parses supported application-profile ASDUs as ZCL, and recognizes
Keep-Alive traffic before ZCL parsing. APS reassembly and security processing have already happened
before the hardware backend emits the indication. Before forwarding parsed indications to the
protocol actors, the mux normalizes only the backend-defined timestamp and device-key-pair handle
to `()`; APS addressing, profile, cluster, status, security mode, key index, link quality, and the
parsed ASDU remain attached. The mux selects and parses ZCL, ZDP, and Keep-Alive payloads directly
from that metadata and ASDU; it does not synthesize a legacy received APS header.

Unmatched ZCL commands remain application-visible as normalized
`DataIndication<Frame<Cluster>, (), ()>` values, preserving the APSDE receive metadata with the
parsed ZCL frame. Supported device notifications also remain application-visible. The coordinator
does not maintain a persistent device table. Application-event delivery uses non-blocking channel
sends. A new event is dropped and logged if the application channel is full or closed, ensuring
application backpressure cannot stall the mux or protocol actors. Applications must therefore
treat events as lossy notifications rather than durable state.

Closure of the hardware event stream is a fatal runtime boundary. The mux sends a terminal message
through each APS, ZCL, ZDP, and OTA inbox. APS fails pending confirmations, ZCL and ZDP fail their
correlation registries, and OTA fails active destination transfers before the actors stop. The mux
also emits `NetworkError::HardwareEventStreamClosed` to the application. Existing handles cannot
restart these actors; the application must construct a new coordinator with a live hardware event
stream.

## Public Trait Composition

```mermaid
flowchart TD
    C[Coordinator]
    N[NCP helpers]
    ZCL[Zcl]
    ZDP[Zdp]
    CL[OnOff ColorControl Level Attributes]
    DS[Node Endpoints Binding Leaving]
    ZCLR[ZclResponse]
    ZDPR[ZdpResponse]

    C --> N
    C --> ZCL
    C --> ZDP
    ZCL --> CL
    ZDP --> DS
    ZCL -->|communicate| ZCLR
    ZDP -->|communicate| ZDPR
```

Command helpers that do not expect a protocol response return `Result<(), Error>` after completing
the deferred APS result outside the protocol actor. Communication methods return `ZclResponse<T>`
or `ZdpResponse<T>` containing both the deferred APS completion and the application-level response.
