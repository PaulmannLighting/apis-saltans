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

/// Messages exchanged with the NCP driver actor.
pub enum Message {
    /// Return the transaction sequence number.
    GetTransactionSeq { response: Sender<u8> },
    /// Return the PAN ID.
    GetPanId {
        response: Sender<Result<u16, Error>>,
    },
    /// Return the IEEE address of the coordinator.
    GetIeeeAddress {
        response: Sender<Result<MacAddr8, Error>>,
    },
    /// Scan for networks.
    ScanNetworks {
        channel_mask: u32,
        duration: u8,
        response: Sender<Result<Vec<FoundNetwork>, Error>>,
    },
    /// Scan Zigbee channels.
    ScanChannels {
        channel_mask: u32,
        duration: u8,
        response: Sender<Result<Vec<ScannedChannel>, Error>>,
    },
    /// Allow devices to join the network.
    AllowJoins {
        duration: Duration,
        response: Sender<Result<(), Error>>,
    },
    /// Get the neighbor table entries.
    GetNeighbors {
        response: Sender<Result<BTreeMap<MacAddr8, u16>, Error>>,
    },
    /// Send a route request.
    RouteRequest {
        radius: u8,
        response: Sender<Result<(), Error>>,
    },
    /// Return the IEEE address corresponding to a short ID.
    TranslateIeeeAddress {
        short_id: u16,
        response: Sender<Result<MacAddr8, Error>>,
    },
    /// Return the short ID corresponding to an IEEE address.
    TranslateShortId {
        ieee_address: MacAddr8,
        response: Sender<Result<u16, Error>>,
    },
    /// Send a unicast request.
    Unicast {
        short_id: u16,
        endpoint: Endpoint,
        frame: Frame,
        response: Sender<Result<u8, Error>>,
    },
    /// Send a multicast request.
    Multicast {
        group_id: u16,
        hops: u8,
        radius: u8,
        frame: Frame,
        response: Sender<Result<u8, Error>>,
    },
    /// Send a broadcast request.
    Broadcast {
        short_id: u16, // TODO: This must be a Zigbee broadcast address. Maybe introduce a new type?
        radius: u8,
        frame: Frame,
        response: Sender<Result<u8, Error>>,
    },
}
