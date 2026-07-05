//! Attributes of the Groups cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::Type;

use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: ClusterId::Groups;

    /// Flag indicating whether the group name is supported by the device.
    NameSupport = 0x0000: Type { R },
}
