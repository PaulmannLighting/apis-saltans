//! Attributes of the Ballast Configuration cluster.

use apis_saltans_core::Cluster;
use apis_saltans_core::types::{String as ZclString, Uint8, Uint16, Uint24};

pub use self::types::LampAlarmMode;
use super::BallastStatus;
use super::ballast_settings_attribute::Level;
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: Cluster::BallastConfiguration;

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
    /// Level used when power is applied to the ballast.
    PowerOnLevel = 0x0012: Uint8 { R, W },
    /// Time used to fade to `PowerOnLevel` when power is applied to the ballast.
    PowerOnFadeTime = 0x0013: Uint16 { R, W },
    /// Intrinsic ballast factor of the ballast/lamp combination.
    IntrinsicBallastFactor = 0x0014: Uint8 { R, W },
    /// Adjustment factor applied to the configured light output.
    BallastFactorAdjustment = 0x0015: Uint8 { R, W },
    /// Number of lamps connected to this ballast.
    LampQuantity = 0x0020: Uint8 { R },
    /// Type of lamps connected to this ballast.
    LampType = 0x0030: ZclString<16> { R, W },
    /// Manufacturer of the connected lamps.
    LampManufacturer = 0x0031: ZclString<16> { R, W },
    /// Rated lamp operating hours.
    LampRatedHours = 0x0032: Uint24 { R, W },
    /// Cumulative lamp burn hours since last re-lamping.
    LampBurnHours = 0x0033: Uint24 { R, W },
    /// Lamp alarms that may be generated.
    LampAlarmMode = 0x0034: LampAlarmMode { R, W },
    /// Lamp burn hours threshold for generating an alarm.
    LampBurnHoursTripPoint = 0x0035: Uint24 { R, W },
}
