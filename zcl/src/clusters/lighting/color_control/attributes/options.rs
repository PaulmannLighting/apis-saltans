use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};
use zb_core::types::Type;

/// Options for the On/Off cluster commands.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct Options(u8);

bitflags! {
    impl Options: u8 {
        /// Execute command if, in the On/Off cluster, the OnOff attribute is `0x00` (`FALSE`).
        const ExecuteIfOff = 0b0000_0001;
    }
}

impl From<Options> for Type {
    fn from(value: Options) -> Self {
        Self::Map8(value.bits())
    }
}

impl TryFrom<Type> for Options {
    type Error = Type;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        if let Type::Map8(value) = value {
            Ok(Self::from_bits_retain(value))
        } else {
            Err(value)
        }
    }
}
