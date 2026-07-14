use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};
use zb_core::types::Type;

/// Zone status attributes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct Status(u16);

bitflags! {
    impl Status: u16 {
        /// 1 – opened or alarmed, 0 – closed or not alarmed
        const ALARM_1 = 0b0000_0000_0000_0001;

        /// 1 – opened or alarmed, 0 – closed or not alarmed
        const ALARM_2 = 0b0000_0000_0000_0010;

        /// 1 – tamper detected, 0 – tamper not detected
        const TAMPER = 0b0000_0000_0000_0100;

        /// 1 – battery low, 0 – battery normal
        const BATTERY = 0b0000_0000_0000_1000;

        /// 1 – supervision notification, 0 – no supervision notification
        const SUPERVISION_NOTIFY = 0b0000_0000_0001_0000;

        /// 1 – notify restore, 0 – no notify restore
        const RESTORE_NOTIFY = 0b0000_0000_0010_0000;

        /// 1 – trouble detected, 0 – no trouble detected
        const TROUBLE = 0b0000_0000_0100_0000;

        /// 1 – AC / Mains fault, 0 – no AC / Mains fault
        const AC_MAINS = 0b0000_0000_1000_0000;

        /// 1 – Sensor is in test mode, 0 – Sensor is not in test mode
        const TEST = 0b0000_0001_0000_0000;

        /// 1 – Sensor battery defect, 0 – no sensor battery defect
        const BATTERY_DEFECT = 0b0000_0010_0000_0000;
    }
}

impl From<Status> for Type {
    fn from(value: Status) -> Self {
        Self::Map16(value.bits().into())
    }
}

impl TryFrom<Type> for Status {
    type Error = Type;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        if let Type::Map16(value) = value {
            Ok(Self::from_bits_retain(value.into_inner()))
        } else {
            Err(value)
        }
    }
}
