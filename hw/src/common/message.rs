use std::collections::BTreeMap;
use std::time::Duration;

use tokio::sync::oneshot::Sender;
use zb_core::{Application, Destination, IeeeAddress};

pub use self::found_network::{FoundNetwork, Network};
pub use self::scanned_channel::ScannedChannel;
use crate::common::{Datagram, Error};
use crate::{Clusters, HwResponse};

mod found_network;
mod scanned_channel;

/// A handle on the NCP.
pub type NcpHandle = tokio::sync::mpsc::Sender<Message>;

/// A weak handle on the NCP.
pub type WeakNcpHandle = tokio::sync::mpsc::WeakSender<Message>;

/// Messages exchanged with the NCP driver actor.
pub enum Message {
    /// Return local application endpoint cluster sets.
    GetEndpoints {
        /// One-shot channel used to return application endpoint cluster sets or driver error.
        response: Sender<Result<BTreeMap<Application, Clusters>, Error>>,
    },

    /// Return the PAN ID.
    GetPanId {
        /// One-shot channel used to return the PAN ID or driver error.
        response: Sender<Result<u16, Error>>,
    },

    /// Return the IEEE address of the coordinator.
    GetIeeeAddress {
        /// One-shot channel used to return the IEEE address or driver error.
        response: Sender<Result<IeeeAddress, Error>>,
    },

    /// Scan for networks.
    ScanNetworks {
        /// Bit mask selecting the Zigbee channels to scan.
        channel_mask: u32,
        /// Backend-specific scan duration exponent.
        duration: u8,
        /// One-shot channel used to return discovered networks or driver error.
        response: Sender<Result<Vec<FoundNetwork>, Error>>,
    },

    /// Scan Zigbee channels.
    ScanChannels {
        /// Bit mask selecting the Zigbee channels to scan.
        channel_mask: u32,
        /// Backend-specific scan duration exponent.
        duration: u8,
        /// One-shot channel used to return channel scan results or driver error.
        response: Sender<Result<Vec<ScannedChannel>, Error>>,
    },

    /// Allow devices to join the network.
    AllowJoins {
        /// Requested permit-join duration.
        duration: Duration,
        /// One-shot channel used to return the actual permit-join duration or driver error.
        response: Sender<Result<Duration, Error>>,
    },

    /// Send a route request.
    RouteRequest {
        /// Maximum route discovery radius.
        radius: u8,
        /// One-shot channel used to return success or driver error.
        response: Sender<Result<(), Error>>,
    },

    /// Return the IEEE address corresponding to a short ID.
    TranslateIeeeAddress {
        /// NWK short ID to resolve.
        short_id: u16,
        /// One-shot channel used to return the IEEE address or driver error.
        response: Sender<Result<IeeeAddress, Error>>,
    },

    /// Return the short ID corresponding to an IEEE address.
    TranslateShortId {
        /// IEEE address to resolve.
        ieee_address: IeeeAddress,
        /// One-shot channel used to return the short ID or driver error.
        response: Sender<Result<u16, Error>>,
    },

    /// Transmit a serialized application datagram.
    Transmit {
        /// APS destination for the datagram.
        destination: Destination,
        /// Serialized payload and APS metadata to transmit.
        datagram: Datagram,
        /// One-shot channel used to return success or driver error.
        response: Sender<Result<HwResponse, Error>>,
    },
}
