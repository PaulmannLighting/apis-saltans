use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

/// Attribute to define the behavior of the On/Off cluster at startup.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum StartUpOnOff {
    /// Set the `OnOff` attribute to 0 (off).
    Off = 0x00,
    /// Set the `OnOff` attribute to 1 (on).
    On = 0x01,
    /// If the previous value of the `OnOff` attribute is equal to 0, set the `OnOff`
    /// attribute to 1. If the previous value of the `OnOff` attribute is equal to 1,
    /// set the `OnOff` attribute to 0 (toggle).
    Toggle = 0x02,
    /// Set the `OnOff` attribute to its previous value.
    Previous = 0xff,
}

impl From<StartUpOnOff> for u8 {
    fn from(start_up_on_off: StartUpOnOff) -> Self {
        start_up_on_off as Self
    }
}

impl TryFrom<u8> for StartUpOnOff {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
