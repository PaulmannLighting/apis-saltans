use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

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
