//! Attributes of the On/Off cluster.

use apis_saltans_core::Cluster;
use apis_saltans_core::types::{Bool, Uint16};

pub use self::types::StartUpOnOff;
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: Cluster::OnOff;

    /// On/Off state of the device.
    OnOff = 0x0000: Bool { R, P, S },
    /// Global scene control.
    GlobalSceneControl = 0x4000: Bool { R },
    /// On time attribute.
    OnTime = 0x4001: Uint16 { R, W },
    /// Off wait time attribute.
    OffWaitTime = 0x4002: Uint16 { R, W },
    /// Behavior of the On/Off cluster at startup.
    StartUpOnOff = 0x4003: StartUpOnOff { R, W },
}
