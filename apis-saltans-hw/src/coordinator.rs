#![cfg(feature = "coordinator")]

//! Coordinator-facing hardware abstraction API.

pub use self::await_event::AwaitEvent;
pub use self::ncp::Ncp;
use crate::common::message::Message;

/// A weak handle on the NCP.
pub type WeakNcpHandle = tokio::sync::mpsc::WeakSender<Message>;

mod await_event;
mod ncp;
