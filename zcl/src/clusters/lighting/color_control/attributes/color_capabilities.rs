use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};
use zb_core::types::Type;

/// Color-related capabilities of a device.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct ColorCapabilities(u16);

impl zb_core::TypeId for ColorCapabilities {
    const ID: u8 = <zb_core::types::Map16 as zb_core::TypeId>::ID;
}

bitflags! {
    impl ColorCapabilities: u16 {
        /// Indicates that the device supports huw and saturation.
        const HueSaturationSupported = 0b0000_0000_0000_0001;
        /// Indicates that the device supports enhanced hue.
        const EnhancedHueSupported = 0b0000_0000_0000_0010;
        /// Indicates that the device supports color loop.
        const ColorLoopSupported = 0b0000_0000_0000_0100;
        /// Indicates that the device supports X/Y color values.
        const XyAttributesSupported = 0b0000_0000_0000_1000;
        /// Indicates that the device supports color temperature.
        const ColorTemperatureSupported = 0b0000_0000_0001_0000;
    }
}

crate::macros::impl_bitflags_display_and_from_str!(ColorCapabilities);

impl From<ColorCapabilities> for Type {
    fn from(value: ColorCapabilities) -> Self {
        Self::Map16(value.bits().into())
    }
}

impl TryFrom<Type> for ColorCapabilities {
    type Error = Type;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        if let Type::Map16(value) = value {
            Ok(Self::from_bits_retain(value.into_inner()))
        } else {
            Err(value)
        }
    }
}
