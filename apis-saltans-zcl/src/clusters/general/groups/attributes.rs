//! Attributes of the Groups cluster.

use apis_saltans_core::Cluster;

pub use self::types::NameSupport;
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: Cluster::Groups;

    /// Flag indicating whether the group name is supported by the device.
    NameSupport = 0x0000: NameSupport { R },
}
