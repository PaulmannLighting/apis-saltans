//! Zigbee hardware API.
//!
//! This library provides a unified interface to implement Zigbee coordinator functionality for
//! Zigbee hardware (NCP) drivers.

pub use event_translator::EventTranslator;
use tokio::sync::mpsc::{Sender, WeakSender};

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

/// A weak handle on the NCP.
pub type WeakNcpHandle = WeakSender<Message>;

mod await_event;
mod bridge;
mod error;
mod event;
mod event_translator;
mod frame;
mod message;
mod ncp;
mod ncp_driver;
mod start;
