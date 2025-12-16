use std::collections::BTreeMap;
use std::time::Duration;

use macaddr::MacAddr8;
use tokio::sync::oneshot::Sender;
use zigbee::Endpoint;

pub use self::found_network::{FoundNetwork, Network};
pub use self::scanned_channel::ScannedChannel;
use crate::{Error, Frame};

mod found_network;
mod scanned_channel;

/// Messages sent to the NWK actor.
pub enum Message {
    GetTransactionSeq {
        response: Sender<u8>,
    },
    GetPanId {
        response: Sender<Result<u16, Error>>,
    },
    ScanNetworks {
        channel_mask: u32,
        duration: u8,
        response: Sender<Result<Vec<FoundNetwork>, Error>>,
    },
    ScanChannels {
        channel_mask: u32,
        duration: u8,
        response: Sender<Result<Vec<ScannedChannel>, Error>>,
    },
    AllowJoins {
        duration: Duration,
        response: Sender<Result<(), Error>>,
    },
    GetNeighbors {
        response: Sender<Result<BTreeMap<MacAddr8, u16>, Error>>,
    },
    RouteRequest {
        radius: u8,
        response: Sender<Result<(), Error>>,
    },
    Unicast {
        pan_id: u16,
        endpoint: Endpoint,
        frame: Frame,
        response: Sender<Result<(), Error>>,
    },
    Multicast {
        group_id: u16,
        hops: u8,
        radius: u8,
        frame: Frame,
        response: Sender<Result<(), Error>>,
    },
    Broadcast {
        pan_id: u16,
        radius: u8,
        frame: Frame,
        response: Sender<Result<(), Error>>,
    },
}
