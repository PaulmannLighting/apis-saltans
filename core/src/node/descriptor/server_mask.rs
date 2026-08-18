use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// The server mask field of the node descriptor is sixteen bits in length,
/// with bit settings signifying the system server capabilities of this node.
///
/// It is used to facilitate discovery of particular system servers by other nodes on the system.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ServerMask(u16);

// Zigbee stores server capabilities in bits 0-6 and the compliance revision in bits 9-15.
bitflags! {
    impl ServerMask: u16 {
        /// Primary Trust Center
        const PRIMARY_TRUST_CENTER = 0b0000_0000_0000_0001;
        /// Backup Trust Center
        const BACKUP_TRUST_CENTER = 0b0000_0000_0000_0010;
        /// Network Manager
        const NETWORK_MANAGER = 0b0000_0000_0100_0000;
        /// Stack Compliance Revision
        const STACK_COMPLIANCE_REVISION = 0b1111_1110_0000_0000;
    }
}

impl_bitflags_display_and_from_str!(ServerMask);

impl ServerMask {
    /// Return the stack compliance revision.
    #[must_use]
    pub const fn stack_compliance_revision(self) -> u8 {
        ((self.0 & Self::STACK_COMPLIANCE_REVISION.bits()) >> 9) as u8
    }

    /// Set the stack compliance revision.
    pub fn set_stack_compliance_revision(&mut self, revision: u8) {
        *self = (*self & !Self::STACK_COMPLIANCE_REVISION)
            | Self(Self::STACK_COMPLIANCE_REVISION.bits() & (u16::from(revision) << 9));
    }
}

// These wire-format tests live beside the implementation because this crate's
// integration-test target inherits every dependency and its strict
// unused-crate-dependencies lint.
#[cfg(test)]
mod tests {
    use le_stream::{FromLeStream, ToLeStream};

    use super::ServerMask;

    #[test]
    fn server_mask_capabilities_when_read_then_match_zigbee_assignments() {
        assert_eq!(ServerMask::PRIMARY_TRUST_CENTER.bits(), 0x0001);
        assert_eq!(ServerMask::BACKUP_TRUST_CENTER.bits(), 0x0002);
        assert_eq!(ServerMask::NETWORK_MANAGER.bits(), 0x0040);
    }

    #[test]
    fn server_mask_when_revision_22_then_matches_zigbee_wire_layout() {
        let mut server_mask = ServerMask::PRIMARY_TRUST_CENTER | ServerMask::NETWORK_MANAGER;

        server_mask.set_stack_compliance_revision(22);

        assert_eq!(server_mask.bits(), 0x2C41);
        assert_eq!(server_mask.stack_compliance_revision(), 22);
        let mut bytes = server_mask.to_le_stream();
        assert_eq!(bytes.next(), Some(0x41));
        assert_eq!(bytes.next(), Some(0x2C));
        assert_eq!(bytes.next(), None);

        assert_eq!(
            ServerMask::from_le_stream([0x41, 0x2C].into_iter()),
            Some(server_mask)
        );
    }
}
