//! Attributes of the Ballast Configuration cluster.

use apis_saltans_core::ClusterId;

use super::BallastStatus;
use super::ballast_settings_attribute::Level;
use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: ClusterId::BallastConfiguration;

    /// Physical minimum level of the ballast.
    PhysicalMinLevel = 0x0000: Level { R },
    /// Physical maximum level of the ballast.
    PhysicalMaxLevel = 0x0001: Level { R },
    /// Status of the ballast.
    BallastStatus = 0x0002: BallastStatus { R },
    /// Minimum light output level.
    MinLevel = 0x0010: Level { R, W },
    /// Maximum light output level.
    MaxLevel = 0x0011: Level { R, W },
}
