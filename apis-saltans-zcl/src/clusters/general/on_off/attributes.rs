//! Attributes of the On/Off cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::{Bool, Type, Uint16};

use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: ClusterId::OnOff;

    /// On/Off state of the device.
    OnOff = 0x0000: Bool { R, W, P, S },
    /// Global scene control.
    GlobalSceneControl = 0x4000: Bool { R },
    /// On time attribute.
    OnTime = 0x4001: Uint16 { R, W },
    /// Off wait time attribute.
    OffWaitTime = 0x4002: Uint16 { R, W },
    /// Behavior of the On/Off cluster at startup.
    StartUpOnOff = 0x4003: Type { R, W },
}
