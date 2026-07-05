//! Attributes of the Scenes cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::{Bool, Type, Uint8};

use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: ClusterId::Scenes;

    /// Number of scenes currently stored in the device.
    SceneCount = 0x0000: Uint8 { R },
    /// ID of the last invoked scene.
    CurrentScene = 0x0001: Uint8 { R },
    /// Group ID of the last invoked scene.
    CurrentGroup = 0x0002: Type { R },
    /// Flag indicating whether the scene is valid.
    SceneValid = 0x0003: Bool { R },
    /// Flag indicating whether the device supports scene names.
    NameSupport = 0x0004: Type { R },
    /// IEEE address of the device that last configured the scene.
    LastConfiguredBy = 0x0005: Type { R },
}
