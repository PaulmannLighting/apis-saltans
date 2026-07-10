//! Attributes of the IAS Zone cluster.

use zb_core::Cluster;
use zb_core::types::Uint8;

pub use self::types::{IasCieAddress, ZoneState};
use super::{Status, Type as ZoneType};
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: Cluster::IasZone;

    /// The zone state.
    ZoneState = 0x0000: ZoneState { R },
    /// The zone type.
    ZoneType = 0x0001: ZoneType { R },
    /// The zone status.
    ZoneStatus = 0x0002: Status { R, P },
    /// The address of the IAS CIE device.
    IasCieAddress = 0x0010: IasCieAddress { R, W },
    /// The zone identifier.
    ZoneId = 0x0011: Uint8 { R },
    /// Number of supported zone sensitivity levels.
    NumberOfZoneSensitivityLevelsSupported = 0x0012: Uint8 { R },
    /// Current zone sensitivity level.
    CurrentZoneSensitivityLevel = 0x0013: Uint8 { R, W },
}
