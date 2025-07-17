use mains_alarm_mask::MainsAlarmMask;
use repr_discriminant::repr_discriminant;

mod mains_alarm_mask;

/// Available mains settings attributes.
#[repr_discriminant(u16)]
pub enum MainsSettings {
    /// Mains alarms.
    AlarmMask(MainsAlarmMask) = 0x0010,
    /// Mains voltage minimum threshold in 100mV.
    VoltageMinThreshold(u16) = 0x0011,
    /// Mains voltage maximum threshold in 100mV.
    VoltageMaxThreshold(u16) = 0x0012,
    /// Mains voltage dwell trip point in seconds.
    VoltageDwellTripPoint(u16) = 0x0013,
}
