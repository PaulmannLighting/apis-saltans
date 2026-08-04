use serde::{Deserialize, Serialize};
use zb_aps::apsde::IndividualEndpoint;
use zb_core::{Endpoint, FullAddress, short_id};

/// Addressing information from a received Keep-Alive packet.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "KeepAliveRepr", into = "KeepAliveRepr")]
pub struct KeepAlive {
    device: short_id::Device,
    endpoint: IndividualEndpoint,
}

#[derive(Deserialize, Serialize)]
struct KeepAliveRepr {
    device: short_id::Device,
    endpoint: Endpoint,
}

/// Device lifecycle or activity event.
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

    /// A device sent a Keep-Alive packet from the contained short address and endpoint.
    KeepAlive(KeepAlive),
}

impl KeepAlive {
    /// Create received Keep-Alive addressing information.
    #[must_use]
    pub const fn new(device: short_id::Device, endpoint: IndividualEndpoint) -> Self {
        Self { device, endpoint }
    }

    /// Return the sending device's NWK short address.
    #[must_use]
    pub const fn device(self) -> short_id::Device {
        self.device
    }

    /// Return the sending APS endpoint.
    #[must_use]
    pub const fn endpoint(self) -> IndividualEndpoint {
        self.endpoint
    }
}

impl From<KeepAlive> for KeepAliveRepr {
    fn from(keep_alive: KeepAlive) -> Self {
        Self {
            device: keep_alive.device,
            endpoint: keep_alive.endpoint.get(),
        }
    }
}

impl TryFrom<KeepAliveRepr> for KeepAlive {
    type Error = &'static str;

    fn try_from(keep_alive: KeepAliveRepr) -> Result<Self, Self::Error> {
        let endpoint = IndividualEndpoint::new(keep_alive.endpoint)
            .ok_or("Keep-Alive endpoint must be individual")?;
        Ok(Self::new(keep_alive.device, endpoint))
    }
}
