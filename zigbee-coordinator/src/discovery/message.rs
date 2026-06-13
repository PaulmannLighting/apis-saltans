use zdp::{ActiveEpRsp, SimpleDescRsp};
use zigbee::{Address, Application};
use zigbee_hw::Event;

use crate::Error;

/// A message received by the discovery actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Event(Event),

    /// A device discovery response.
    ActiveEpRsp {
        /// The full address of the device.
        address: Address,
        /// The result of the request.
        result: Result<ActiveEpRsp, Error>,
    },

    /// An endpoint discovery response.
    SimpleDescRsp {
        /// The full address of the device.
        address: Address,
        /// The result of the request.
        result: Result<SimpleDescRsp, Error>,
    },
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}
