//! Attributes of the Scenes cluster.

use apis_saltans_core::Cluster;
use apis_saltans_core::types::{Bool, Uint8};

pub use self::types::{CurrentGroup, LastConfiguredBy};
pub use crate::clusters::general::groups::NameSupport;
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: Cluster::Scenes;

    /// Number of scenes currently stored in the device.
    SceneCount = 0x0000: Uint8 { R },
    /// ID of the last invoked scene.
    CurrentScene = 0x0001: Uint8 { R },
    /// Group ID of the last invoked scene.
    CurrentGroup = 0x0002: CurrentGroup { R },
    /// Flag indicating whether the scene is valid.
    SceneValid = 0x0003: Bool { R },
    /// Flag indicating whether the device supports scene names.
    NameSupport = 0x0004: NameSupport { R },
    /// IEEE address of the device that last configured the scene.
    LastConfiguredBy = 0x0005: LastConfiguredBy { R },
}
