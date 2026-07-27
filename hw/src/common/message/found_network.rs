//! Data structures for network information.

pub use self::network_descriptor::NetworkDescriptor;

/// Network configuration returned by a scan.
mod network_descriptor;

/// A found network with additional link quality information.
///
/// You should implement `From<T> for FoundNetwork` on your
/// implementation-specific _found network_ message type.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FoundNetwork {
    network: NetworkDescriptor,
    last_hop_lqi: u8,
    last_hop_rssi_dbm: i8,
}

impl FoundNetwork {
    /// Create a new `FoundNetwork`.
    #[must_use]
    pub const fn new(network: NetworkDescriptor, last_hop_lqi: u8, last_hop_rssi_dbm: i8) -> Self {
        Self {
            network,
            last_hop_lqi,
            last_hop_rssi_dbm,
        }
    }

    /// Return the discovered network descriptor.
    #[must_use]
    pub const fn network(&self) -> &NetworkDescriptor {
        &self.network
    }

    /// Get the last hop LQI of the found network.
    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    /// Return the last-hop RSSI in dBm.
    #[must_use]
    pub const fn last_hop_rssi_dbm(&self) -> i8 {
        self.last_hop_rssi_dbm
    }
}
