#[cfg(feature = "driver")]
use std::num::NonZeroUsize;
use std::time::Duration;

use bytes::Bytes;
#[cfg(feature = "driver")]
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::{Sender as MpscSender, WeakSender};
use tokio::sync::oneshot::Sender;
use zb_aps::Data;
use zb_core::short_id::Device;
use zb_core::{Destination, IeeeAddress};
use zb_zdp::SimpleDescriptor;

pub use self::channel::Channel;
pub use self::channel_mask::ChannelMask;
pub use self::found_network::{FoundNetwork, NetworkDescriptor};
pub use self::scan_duration::ScanDuration;
pub use self::scanned_channel::ScannedChannel;
use crate::common::Error;

mod channel;
mod channel_mask;
mod found_network;
mod scan_duration;
mod scanned_channel;

/// A handle on the NCP.
#[derive(Clone, Debug)]
pub struct NcpHandle {
    sender: MpscSender<Message>,
}

impl NcpHandle {
    #[cfg(feature = "driver")]
    pub(crate) fn channel(capacity: NonZeroUsize) -> (Self, tokio::sync::mpsc::Receiver<Message>) {
        let (sender, receiver) = channel(capacity.get());
        let handle = Self { sender };
        (handle, receiver)
    }

    #[cfg(feature = "coordinator")]
    pub(crate) async fn send(&self, message: Message) -> Result<(), Error> {
        Ok(self.sender.send(message).await?)
    }

    /// Create a weak handle that does not keep the driver actor channel open.
    #[must_use]
    pub fn downgrade(&self) -> WeakNcpHandle {
        WeakNcpHandle(self.sender.downgrade())
    }

    /// Return whether the driver actor channel has closed.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }
}

/// A weak handle on the NCP that does not keep its actor channel open.
#[derive(Clone, Debug)]
pub struct WeakNcpHandle(WeakSender<Message>);

impl WeakNcpHandle {
    /// Attempt to upgrade this weak handle.
    #[must_use]
    pub fn upgrade(&self) -> Option<NcpHandle> {
        Some(NcpHandle {
            sender: self.0.upgrade()?,
        })
    }
}

/// Messages exchanged with the NCP driver actor.
#[cfg_attr(
    not(all(feature = "coordinator", feature = "driver")),
    allow(dead_code)
)]
pub enum Message {
    /// Return the NCP's local application endpoint descriptors.
    GetEndpoints {
        /// One-shot channel used to return local simple descriptors or a driver error.
        response: Sender<Result<Box<[SimpleDescriptor]>, Error>>,
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
        channel_mask: ChannelMask,
        /// Scan duration exponent.
        duration: ScanDuration,
        /// One-shot channel used to return discovered networks or driver error.
        response: Sender<Result<Vec<FoundNetwork>, Error>>,
    },

    /// Scan Zigbee channels.
    ScanChannels {
        /// Bit mask selecting the Zigbee channels to scan.
        channel_mask: ChannelMask,
        /// Scan duration exponent.
        duration: ScanDuration,
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
        short_id: Device,
        /// One-shot channel used to return the IEEE address or driver error.
        response: Sender<Result<IeeeAddress, Error>>,
    },

    /// Return the short ID corresponding to an IEEE address.
    TranslateShortId {
        /// IEEE address to resolve.
        ieee_address: IeeeAddress,
        /// One-shot channel used to return the short ID or driver error.
        response: Sender<Result<Device, Error>>,
    },

    /// Transmit an APS data frame.
    Transmit {
        /// Network destination for the frame.
        destination: Destination,
        /// APS frame to transmit.
        frame: Data<Bytes>,
        /// One-shot channel used to report whether the backend accepted the frame.
        response: Sender<Result<(), Error>>,
    },
}
