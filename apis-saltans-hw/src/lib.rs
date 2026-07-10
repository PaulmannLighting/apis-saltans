//! Zigbee hardware abstraction API.
//!
//! This crate defines the boundary between coordinator-level logic and concrete Zigbee network
//! co-processor (NCP) drivers. Driver implementations expose hardware operations through
//! [`Driver`], callers use an [`NcpHandle`] through the [`Ncp`] extension trait, and hardware
//! events are converted into crate-level [`Event`] values by an [`EventTranslator`].

pub use event_translator::EventTranslator;
use tokio::sync::mpsc::{Sender, WeakSender};

pub use self::await_event::AwaitEvent;
pub use self::bridge::bridge;
pub use self::builder::Builder;
pub use self::datagram::{Datagram, Metadata};
pub use self::driver::Driver;
pub use self::error::Error;
pub use self::event::{Event, RouteError};
pub use self::initialize::Initialize;
use self::message::Message;
pub use self::message::{FoundNetwork, Network, ScannedChannel};
pub use self::ncp::Ncp;
pub use self::prepared_hardware::PreparedHardware;

/// A handle on the NCP.
pub type NcpHandle = Sender<Message>;

/// A weak handle on the NCP.
pub type WeakNcpHandle = WeakSender<Message>;

mod await_event;
mod bridge;
mod builder;
mod datagram;
mod driver;
mod error;
mod event;
mod event_translator;
mod initialize;
mod message;
mod ncp;
mod prepared_hardware;
