mod message;

use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use zigbee_hw::Event;

/// The network management actor.
pub struct Actor {}

#[derive(Debug)]
pub struct Device {}
