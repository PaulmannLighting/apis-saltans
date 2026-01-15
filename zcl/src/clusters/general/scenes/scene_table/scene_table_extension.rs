use core::iter::Empty;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::{Bool, Uint8, Uint16};

/// Scene table extension field set.
///
/// TODO: The possible extensions depend on the zcl supported by the device.
/// - Group extensions by cluster.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SceneTableExtension {
    /// On/Off state of the device.
    OnOff(Bool),
    /// The current level of the device.
    CurrentLevel(Uint8),
    /// The current X coordinate in the CIE 1931 color space.
    CurrentX(Uint16),
    /// The current Y coordinate in the CIE 1931 color space.
    CurrentY(Uint16),
    /// The enhanced current hue of the light.
    EnhancedCurrentHue(Uint16),
    /// The current saturation of the light.
    CurrentSaturation(Uint8),
    /// Indicates whether the color loop is active.
    ColorLoopActive(Uint8),
    /// The direction of the color loop.
    ColorLoopDirection(Uint8),
    /// Color loop time in seconds.
    ColorLoopTime(Uint16),
    /// The color temperature of the light in mireds.
    ColorTemperatureMireds(Uint16),
    /// Cooling mode setpoint when room is occupied.
    OccupiedCoolingSetpoint(Uint16),
    /// Heating mode setpoint when room is occupied.
    OccupiedHeatingSetpoint(Uint16),
    /// Operating mode of the thermostat.
    SystemMode(u8), // TODO: Use enum
    /// State of a door lock.
    LockState(u8), // TODO: Use enum
    /// The current position of a window barrier device.
    CurrentPositionLiftPercentage(Uint8),
    /// The current tilt position of a window barrier device.
    CurrentPositionTiltPercentage(Uint8),
    /// The position of a window barrier device.
    BarrierPosition(Uint8),
}

impl FromLeStream for SceneTableExtension {
    fn from_le_stream<T>(_bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        todo!("Deserialization of SceneTableExtension is not yet implemented")
    }
}

impl ToLeStream for SceneTableExtension {
    type Iter = Empty<u8>;

    fn to_le_stream(self) -> Self::Iter {
        todo!("Serialization of SceneTableExtension is not yet implemented")
    }
}
