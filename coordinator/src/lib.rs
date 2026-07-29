//! Zigbee transceiver API.
//!
//! This library provides a fully abstracted interface to expose an interface to communicate with
//! a Zigbee transceiver regardless of the underlying hardware.
//!
//! The application supplies a `tokio::sync::mpsc::Sender<Event>` at startup to receive coordinator
//! [`Event`] values. Discovery, binding, address resolution, and persistence are application-owned
//! workflows built from traits such as [`Node`], [`Endpoints`], [`Binding`],
//! [`AddressTranslation`], [`Zcl`], and [`Zdp`].
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
    AddressTranslation, Attributes, Binding, Channel, ChannelMask, ColorControl, Endpoints,
    FoundNetwork, Groups, Joining, Level, LocalNode, NetworkDescriptor, Node, OnOff, Ota,
    ReadAttributeResult, Routing, ScanDuration, ScannedChannel, Scanning, SimpleDescriptor,
    WriteAttributeResult, Zcl, ZclResponse, Zdp, ZdpResponse,
};
pub use self::coordinator::Coordinator;
pub use self::error::{Error, Optional, StatusExt};
pub use self::event::{Device, Event, Network, NetworkError};
pub use self::ota::{
    BaseHeaderBytes as OtaBaseHeaderBytes, FieldControl as OtaFieldControl, Header as OtaHeader,
    HeaderString as OtaHeaderString, Image as OtaImage, Message as OtaMessage, ParseImage,
    ParseImageError, UpdateError as OtaUpdateError, UpdateResult as OtaUpdateResult,
};
pub use self::response::CommunicationResponse;

mod api;
mod aps;
mod apsde;
mod coordinator;
mod error;
mod event;
mod index;
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
