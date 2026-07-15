use serde::{Deserialize, Serialize};
use zb_core::FullAddress;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Device {
    /// A device joined the network.
    Joined(FullAddress),

    Rejoined {
        address: FullAddress,
        secured: bool,
    },

    /// A device left the network.
    Left(FullAddress),

    /// A device announced itself on the network.
    Announced(FullAddress),
}
