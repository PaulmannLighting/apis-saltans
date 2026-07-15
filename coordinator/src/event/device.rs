use serde::{Deserialize, Serialize};
use zb_core::FullAddress;

/// Device lifecycle event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Device {
    /// A device joined the network.
    Joined(FullAddress),

    /// A device rejoined the network.
    Rejoined {
        /// Full address of the rejoined device.
        address: FullAddress,

        /// Whether the rejoin was secured.
        secured: bool,
    },

    /// A device left the network.
    Left(FullAddress),

    /// A device announced itself on the network.
    Announced(FullAddress),
}
