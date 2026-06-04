//! Zigbee hardware API.
//!
//! This library provides a unified interface to implement Zigbee coordinator functionality for
//! Zigbee hardware (NCP) drivers.

use tokio::sync::mpsc::Sender;

pub use self::await_event::AwaitEvent;
pub use self::bridge::bridge;
pub use self::error::Error;
pub use self::event::{Command, Event};
pub use self::frame::{Frame, Metadata};
pub use self::message::{FoundNetwork, Network, ScannedChannel};
pub use self::ncp::Ncp;
pub use self::ncp_driver::NcpDriver;
pub use self::start::Start;
use crate::message::Message;

/// A handle on the NCP.
pub type NcpHandle = Sender<Message>;

mod await_event;
mod bridge;
mod error;
mod event;
mod frame;
mod message;
mod ncp;
mod ncp_driver;
mod start;
