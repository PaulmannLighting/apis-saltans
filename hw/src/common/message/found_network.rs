//! Data structures for network information.

pub use self::network::Network;

pub mod network;

/// A found network with additional link quality information.
///
/// You should implement `From<T> for FoundNetwork` on your
/// implementation-specific _found network_ message type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FoundNetwork {
    network: Network,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
}

impl FoundNetwork {
    /// Create a new `FoundNetwork`.
    #[must_use]
    pub const fn new(network: Network, last_hop_lqi: u8, last_hop_rssi: i8) -> Self {
        Self {
            network,
            last_hop_lqi,
            last_hop_rssi,
        }
    }

    /// Get the underlying `Network`.
    #[must_use]
    pub const fn network(&self) -> &Network {
        &self.network
    }

    /// Get the last hop LQI of the found network.
    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    /// Get the last hop RSSI of the found network.
    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }
}
