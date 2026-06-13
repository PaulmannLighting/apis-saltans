use zdp::{ActiveEpRsp, SimpleDescRsp};
use zigbee::{Address, Application};
use zigbee_hw::Event;

use crate::Error;
use crate::discovery::endpoint::Attributes;

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

    /// Cluster attribute report.
    Attributes {
        /// The full address of the device.
        address: Address,
        /// The application endpoint.
        endpoint: Application,
        /// The cluster ID.
        cluster_id: u16,
        /// The attributes.
        attributes: Attributes,
    },
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}
