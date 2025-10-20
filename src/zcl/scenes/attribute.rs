use le_stream::derive::FromLeStreamTagged;
use macaddr::MacAddr8;
use repr_discriminant::ReprDiscriminant;

use crate::types::{Bool, Uint8, Uint16};
pub use crate::zcl::groups::NameSupport;

/// Attributes for the Scenes cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
#[allow(variant_size_differences)]
pub enum Attribute {
    /// Number of scenes currently stored in the device.
    SceneCount(Uint8) = 0x0000,
    /// ID of the last invoked scene.
    CurrentScene(Uint8) = 0x0001,
    /// Group ID of the last invoked scene.
    CurrentGroup(Uint16) = 0x0002, // TODO: Limit to 0xfff7
    /// Flag indicating whether the scene is valid.
    SceneValid(Bool) = 0x0003,
    /// Flag indicating whether the device supports scene names.
    NameSupport(NameSupport) = 0x0004,
    /// IEEE address of the device that last configured the scene.
    LastConfiguredBy(MacAddr8) = 0x0005,
}
