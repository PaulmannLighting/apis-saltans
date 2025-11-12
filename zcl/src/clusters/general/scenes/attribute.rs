use le_stream::derive::FromLeStreamTagged;
use macaddr::MacAddr8;
use repr_discriminant::ReprDiscriminant;
use zigbee::Parsable;
use zigbee::types::{Bool, Uint8};

use super::types::CurrentGroup;
pub use crate::clusters::general::groups::NameSupport;

/// Attributes for the Scenes cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
#[expect(variant_size_differences)]
pub enum Attribute {
    /// Number of scenes currently stored in the device.
    SceneCount(Uint8) = 0x0000,
    /// ID of the last invoked scene.
    CurrentScene(Uint8) = 0x0001,
    /// Group ID of the last invoked scene.
    CurrentGroup(CurrentGroup) = 0x0002,
    /// Flag indicating whether the scene is valid.
    SceneValid(Bool) = 0x0003,
    /// Flag indicating whether the device supports scene names.
    NameSupport(Parsable<u8, NameSupport>) = 0x0004,
    /// IEEE address of the device that last configured the scene.
    LastConfiguredBy(MacAddr8) = 0x0005,
}
