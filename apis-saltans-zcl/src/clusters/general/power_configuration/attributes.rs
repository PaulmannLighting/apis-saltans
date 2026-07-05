//! Attributes of the Power Configuration cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::{Type, Uint8, Uint16};

use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: ClusterId::PowerConfiguration;

    /// Mains voltage in 100mV.
    MainsVoltage = 0x0000: Uint16 { R, W, P },
    /// Mains frequency in Hertz.
    MainsFrequency = 0x0001: Uint8 { R, W, P },
    /// Mains alarms.
    AlarmMask = 0x0010: Type { R, W, P },
    /// Mains voltage minimum threshold in 100mV.
    VoltageMinThreshold = 0x0011: Uint16 { R, W, P },
    /// Mains voltage maximum threshold in 100mV.
    VoltageMaxThreshold = 0x0012: Uint16 { R, W, P },
    /// Mains voltage dwell trip point in seconds.
    VoltageDwellTripPoint = 0x0013: Uint16 { R, W, P },
    /// Primary battery data.
    Battery = 0x0020: Type { R, W, P },
    /// Secondary battery data.
    Battery2 = 0x0040: Type { R, W, P },
    /// Tertiary battery data.
    Battery3 = 0x0060: Type { R, W, P },
}
