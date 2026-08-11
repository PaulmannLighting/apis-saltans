//! Zigbee transceiver API.
//!
//! This library provides a fully abstracted interface to expose an interface to communicate with
//! a Zigbee transceiver regardless of the underlying hardware.
//!
//! The application supplies a `tokio::sync::mpsc::Sender<Event>` at startup to receive coordinator
//! [`Event`] values. Delivery is non-blocking: an event is dropped if that channel is full or
//! closed, so application backpressure cannot stall protocol processing. Discovery, binding,
//! address resolution, and persistence are application-owned workflows built from traits such as
//! [`Node`], [`Endpoints`], [`Binding`], [`Leaving`], [`AddressTranslation`], [`Zcl`], and [`Zdp`].
//! Closing the hardware event stream is fatal: protocol actors fail pending work and stop, and the
//! coordinator emits [`NetworkError::HardwareEventStreamClosed`]. Applications must start a new
//! coordinator with a live hardware event stream after that boundary.
//! The built-in [`Ota`] service validates complete OTA image files and automatically serves the
//! OTA Upgrade cluster exchange for individually scheduled device endpoints.
//!
//! The hardware NCP is responsible for providing its complete local endpoint descriptors through
//! [`zb_hw::NcpHandle::get_endpoints`]. The coordinator queries those descriptors when serving ZDP
//! match requests and exposes them through [`LocalNode::get_endpoints`]. ZCL callers select their
//! source endpoint explicitly in an [`zb_aps::apsde::DataRequest`].
//!
//! ZCL transmissions await a deferred APS completion outside the protocol actor. ZCL and ZDP
//! communication methods return a protocol-specific [`ZclResponse`] or [`ZdpResponse`] that first
//! completes APS transmission and then waits for the correlated command. All operations report
//! failures through the coordinator's [`Error`] type.

use const_env::env_item;

pub use self::api::{
    AddressTranslation, Attributes, Binding, CancellableOtaUpdate, Channel, ChannelMask,
    ColorControl, Endpoints, FoundNetwork, Groups, Joining, Leaving, Level, LocalNode,
    NetworkDescriptor, Node, OnOff, Ota, ReadAttributeResult, Routing, ScanDuration,
    ScannedChannel, Scanning, SimpleDescriptor, WriteAttributeResult, Zcl, ZclResponse, Zdp,
    ZdpResponse,
};
pub use self::coordinator::Coordinator;
pub use self::error::{Error, Optional, StatusExt};
pub use self::event::{Device, Event, KeepAlive, Network, NetworkError};
pub use self::ota::{
    BaseHeaderBytes as OtaBaseHeaderBytes, FieldControl as OtaFieldControl, Header as OtaHeader,
    HeaderString as OtaHeaderString, Image as OtaImage, Message as OtaMessage, ParseImage,
    ParseImageError, UpdateError as OtaUpdateError, UpdateResult as OtaUpdateResult,
    UpdateTimeouts as OtaUpdateTimeouts,
};
pub use self::response::CommunicationResponse;

mod api;
mod aps;
mod coordinator;
mod correlation;
mod error;
mod event;
mod mux;
pub mod ota;
mod response;
mod zcl;
mod zdp;

/// Capacity of each coordinator actor inbox.
#[env_item("ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE")]
const MPSC_CHANNEL_SIZE: usize = 128;

/// Default maximum number of concurrent destination OTA transfer tasks.
const DEFAULT_OTA_UPDATE_TASK_LIMIT: usize = MPSC_CHANNEL_SIZE;
