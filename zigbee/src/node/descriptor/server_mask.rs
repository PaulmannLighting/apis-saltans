use std::fmt::Display;

use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// The server mask field of the node descriptor is sixteen bits in length,
/// with bit settings signifying the system server capabilities of this node.
///
/// It is used to facilitate discovery of particular system servers by other nodes on the system.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct ServerMask(u16);

bitflags! {
    impl ServerMask: u16 {
        /// Primary Trust Center
        const PRIMARY_TRUST_CENTER = 0b1000_0000_0000_0000;
        /// Backup Trust Center
        const BACKUP_TRUST_CENTER = 0b0100_0000_0000_0000;
        /// Primary Binding Table Cache
        const PRIMARY_BINDING_TABLE_CACHE = 0b0010_0000_0000_0000;
        /// Backup Binding Table Cache
        const BACKUP_BINDING_TABLE_CACHE = 0b0001_0000_0000_0000;
        /// Primary Discovery Cache
        const PRIMARY_DISCOVERY_CACHE = 0b0000_1000_0000_0000;
        /// Backup Discovery Cache
        const BACKUP_DISCOVERY_CACHE = 0b0000_0100_0000_0000;
        /// Network Manager
        const NETWORK_MANAGER = 0b0000_0010_0000_0000;
        /// User Descriptor Server
        const RESERVED = 0b0000_0001_1000_0000;
        /// Stack Compliance Revision
        const STACK_COMPLIANCE_REVISION = 0b0000_0000_0111_1111;
    }
}

impl ServerMask {
    /// Return the stack compliance revision.
    #[expect(clippy::cast_possible_truncation)]
    #[must_use]
    pub const fn stack_compliance_revision(self) -> u8 {
        (self.0 & Self::STACK_COMPLIANCE_REVISION.bits()) as u8
    }
}

impl Display for ServerMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (name, flag) in self.iter_names().filter_map(|(name, flag)| {
            if flag == Self::STACK_COMPLIANCE_REVISION {
                None
            } else {
                Some((name, flag))
            }
        }) {
            write!(f, "{name} ({flag:#06X}), ")?;
        }

        write!(
            f,
            "STACK_COMPLIANCE_REVISION = {}]",
            self.stack_compliance_revision()
        )
    }
}
