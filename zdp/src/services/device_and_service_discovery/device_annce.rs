use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;
use zigbee::Cluster;
use zigbee::node::MacCapabilityFlags;

use crate::Service;

/// Device Announcement Service.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct DeviceAnnce {
    nwk_addr: u16,
    ieee_addr: MacAddr8,
    capabilities: MacCapabilityFlags,
}

impl DeviceAnnce {
    /// Creates a new `DeviceAnnce` with the given parameters.
    #[must_use]
    pub const fn new(nwk_addr: u16, ieee_addr: MacAddr8, capabilities: MacCapabilityFlags) -> Self {
        Self {
            nwk_addr,
            ieee_addr,
            capabilities,
        }
    }

    /// Returns the network address.
    #[must_use]
    pub const fn nwk_addr(&self) -> u16 {
        self.nwk_addr
    }

    /// Returns the IEEE address.
    #[must_use]
    pub const fn ieee_addr(&self) -> MacAddr8 {
        self.ieee_addr
    }

    /// Returns the capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> MacCapabilityFlags {
        self.capabilities
    }
}

impl Cluster for DeviceAnnce {
    const ID: u16 = 0x0013;
}

impl Service for DeviceAnnce {
    const NAME: &'static str = "Device_annce";
}

impl Display for DeviceAnnce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ nwk_addr: {:#06X}, ieee_addr: {}, capabilities: {} }}",
            Self::NAME,
            self.nwk_addr,
            self.ieee_addr,
            self.capabilities
        )
    }
}
