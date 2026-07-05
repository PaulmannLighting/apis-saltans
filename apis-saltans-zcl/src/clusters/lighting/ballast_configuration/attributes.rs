//! Attributes of the Ballast Configuration cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::Type;

use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: ClusterId::BallastConfiguration;

    /// Physical minimum level of the ballast.
    PhysicalMinLevel = 0x0000: Type { R },
    /// Physical maximum level of the ballast.
    PhysicalMaxLevel = 0x0001: Type { R },
    /// Status of the ballast.
    BallastStatus = 0x0002: Type { R },
    /// Minimum light output level.
    MinLevel = 0x0010: Type { R, W },
    /// Maximum light output level.
    MaxLevel = 0x0011: Type { R, W },
}
