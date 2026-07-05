//! Attributes of the IAS Zone cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::{Type, Uint8};

use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: ClusterId::IasZone;

    /// The zone state.
    ZoneState = 0x0000: Type { R },
    /// The zone type.
    ZoneType = 0x0001: Type { R },
    /// The zone status.
    ZoneStatus = 0x0002: Type { R, P },
    /// The address of the IAS CIE device.
    IasCieAddress = 0x0010: Type { R, W },
    /// The zone identifier.
    ZoneId = 0x0011: Uint8 { R },
    /// Number of supported zone sensitivity levels.
    NumberOfZoneSensitivityLevelsSupported = 0x0012: Uint8 { R },
    /// Current zone sensitivity level.
    CurrentZoneSensitivityLevel = 0x0013: Uint8 { R, W },
}
