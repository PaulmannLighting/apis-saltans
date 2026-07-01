use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Cluster, Direction};
use le_stream::{FromLeStream, ToLeStream};

use crate::ias::zone::Status;
use crate::{Cluster as ZclCluster, Command};

/// Zone status change attributes.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream)]
pub struct StatusChange {
    status: Status,
    extended_status: u8,
    zone_id: u8,
    delay: Uint16,
}

impl StatusChange {
    /// Create a new status change command.
    #[must_use]
    pub const fn new(status: Status, extended_status: u8, zone_id: u8, delay: Uint16) -> Self {
        Self {
            status,
            extended_status,
            zone_id,
            delay,
        }
    }

    /// Return the status.
    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    /// Return the extended status.
    #[must_use]
    pub const fn extended_status(&self) -> u8 {
        self.extended_status
    }

    /// Return the zone ID.
    #[must_use]
    pub const fn zone_id(&self) -> u8 {
        self.zone_id
    }
}

impl Cluster<ClusterId> for StatusChange {
    const ID: ClusterId = ClusterId::IasZone;
}

impl Command for StatusChange {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ServerToClient;
}

impl From<StatusChange> for ZclCluster {
    fn from(value: StatusChange) -> Self {
        Self::IasZone(value.into())
    }
}
