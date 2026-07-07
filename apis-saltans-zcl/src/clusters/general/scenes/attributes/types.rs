//! Attribute value types of the Scenes cluster.

use apis_saltans_core::IeeeAddress;
use apis_saltans_core::types::Uint16;

use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// An identifier for the current group.
    pub struct CurrentGroup(Uint16) => Uint16;
}

zcl_attribute_newtype! {
    /// IEEE address of the device that last configured the scene.
    pub struct LastConfiguredBy(IeeeAddress) => IeeeAddress;
}
