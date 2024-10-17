use bitflags::bitflags;
use serde::{Deserialize, Serialize};

/// The server mask field of the node descriptor is sixteen bits in length,
/// with bit settings signifying the system server capabilities of this node.
///
/// It is used to facilitate discovery of particular system servers by other nodes on the system.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct ServerMask(u16);

bitflags! {
    impl ServerMask: u16 {
        const PRIMARY_TRUST_CENTER = 0b1000_0000_0000_0000;
        const BACKUP_TRUST_CENTER = 0b0100_0000_0000_0000;
        const PRIMARY_BINDING_TABLE_CACHE = 0b0010_0000_0000_0000;
        const BACKUP_BINDING_TABLE_CACHE = 0b0001_0000_0000_0000;
        const PRIMARY_DISCOVERY_CACHE = 0b0000_1000_0000_0000;
        const BACKUP_DISCOVERY_CACHE = 0b0000_0100_0000_0000;
        const NETWORK_MANAGER = 0b0000_0010_0000_0000;
        const RESERVED = 0b0000_0001_1000_0000;
        const STACK_COMPLIANCE_REVISION = 0b0000_0000_0111_1111;
    }
}
