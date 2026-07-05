# apis-saltans-zdp

Zigbee Device Profile (ZDP) command and service definitions.

This crate provides typed request/response models for ZDP services, grouped command enums, command dispatch by cluster ID,
and frame wrappers with sequence numbers.

## Status

This crate is under active development and does not yet implement the full ZDP command space.

## What This Crate Provides

- ZDP frame wrapper:
  - `Frame<T>` (`seq` + typed payload)
- Service command families:
  - `DeviceAndServiceDiscovery`
  - `BindManagement`
  - `NetworkManagement`
  - `Security`
- Unified command enum:
  - `Command` (all currently supported ZDP commands)
- Descriptor and status types:
  - `SimpleDescriptor`, `AppFlags`, `Clusters`
  - `Status`, `Displayable`
- Service metadata trait:
  - `Service` (`NAME` constants)

## Crate Layout

- `services`: concrete ZDP request/response structures and grouped enums
- `frame`: sequence-numbered ZDP frame representation
- `simple_descriptor`: typed simple descriptor model
- `status`: ZDP status code enum and formatting helpers

## Serialization and Parsing

This crate uses `le-stream` for little-endian wire encoding.

Key flow:
1. Parse the frame sequence byte
2. Parse command payload with known cluster ID
3. Obtain typed `Command`

APIs:
- `Frame<T>: ToLeStream`
- `Frame<Command>::parse_with_cluster_id(cluster_id, bytes)`
- `Command::cluster_id()`
- `Command: ToLeStream`

## Supported Service Groups

### Device and Service Discovery
Includes:
- `NwkAddrReq`, `IeeeAddrReq`
- `NwkAddrRsp`, `IeeeAddrRsp`
- `NodeDescReq`, `NodeDescRsp`
- `PowerDescReq`, `PowerDescRsp`
- `SimpleDescReq`, `SimpleDescRsp`
- `ActiveEpReq`, `ActiveEpRsp`
- `MatchDescReq`, `MatchDescRsp`
- `DeviceAnnce`
- `ParentAnnce`, `ParentAnnceRsp`
- `SystemServerDiscoveryReq`, `SystemServerDiscoveryRsp`

### Bind Management
Includes:
- `BindReq`, `BindRsp`
- `UnbindReq`, `UnbindRsp`
- `ClearAllBindingsReq`, `ClearAllBindingsRsp`

### Network Management
Includes:
- `MgmtLqiReq`, `MgmtLqiRsp`
- `MgmtRtgReq`, `MgmtRtgRsp`
- `MgmtBindReq`, `MgmtBindRsp`
- `MgmtLeaveReq`, `MgmtLeaveRsp`
- `MgmtPermitJoiningReq`, `MgmtPermitJoiningRsp`
- `MgmtNwkUpdateReq`, `MgmtNwkUpdateNotify`
- `MgmtNwkEnhancedUpdateReq`, `MgmtNwkEnhancedUpdateNotify`
- `MgmtNwkIeeeJoiningListReq`, `MgmtNwkIeeeJoiningListRsp`
- `MgmtNwkUnsolicitedEnhancedUpdateNotify`
- `MgmtNwkBeaconSurveyReq`, `MgmtNwkBeaconSurveyRsp`

### Security
Includes:
- `SecurityStartKeyNegotiationReq`, `SecurityStartKeyNegotiationRsp`
- `SecurityRetrieveAuthenticationTokenReq`, `SecurityRetrieveAuthenticationTokenRsp`
- `SecurityGetAuthenticationLevelReq`, `SecurityGetAuthenticationLevelRsp`
- `SecuritySetConfigurationReq`, `SecuritySetConfigurationRsp`
- `SecurityGetConfigurationReq`, `SecurityGetConfigurationRsp`
- `SecurityStartKeyUpdateReq`, `SecurityStartKeyUpdateRsp`
- `SecurityDecommissionReq`, `SecurityDecommissionRsp`
- `SecurityChallengeReq`, `SecurityChallengeRsp`

## Quick Start

### Build and Serialize a ZDP Request Frame

```rust
use le_stream::ToLeStream;
use apis_saltans_zdp::{ActiveEpReq, Command, Frame};

let request = ActiveEpReq::new(0x1234);
let command: Command = request.into();
let frame = Frame::new(0x42, command);

let bytes: Vec<u8> = frame.to_le_stream().collect();
assert!(!bytes.is_empty());
```

### Parse a Frame with Known Cluster ID

```rust
use apis_saltans_core::Cluster;
use apis_saltans_zdp::{ActiveEpReq, Frame};

let raw = vec![0x42, 0x34, 0x12]; // seq + Active_EP_req payload
let parsed = Frame::parse_with_cluster_id(ActiveEpReq::ID, raw.into_iter());
assert!(parsed.is_ok());
```

## Response Coupling and Typed Constructors

Many request types implement `apis_saltans_core::ExpectResponse<apis_saltans_zdp::Command>`, allowing request/response relationships to be expressed in type signatures (for example, `ActiveEpReq -> ActiveEpRsp`, `BindReq -> BindRsp`).

Several response payloads use helper types to encode status-dependent invariants. For example, address and management
responses use typed success payloads when the ZDP status is success and preserve raw status codes when the status cannot
be converted into the crate's `Status` enum.

## Simple Descriptor Support

`SimpleDescriptor` provides typed handling for:
- endpoint
- profile ID / profile conversion
- device ID
- app flags (version extraction)
- input/output cluster lists

This makes it suitable for descriptor transport and higher-level endpoint capability indexing.

## Dependencies

Key dependencies:
- `apis-saltans-core` (cluster IDs, endpoint/profile primitives, shared traits)
- `le-stream`
- `heapless`
- `macaddr`
- `bitflags`

## Related Workspace Crates

- `apis-saltans-core`: core protocol definitions
- `apis-saltans-aps`: APS layer frame structures
- `apis-saltans-zcl`: ZCL command and cluster models
