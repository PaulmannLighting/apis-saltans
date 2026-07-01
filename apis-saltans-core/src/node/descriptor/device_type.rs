/// A device type.
///
/// A Zigbee device is either a full-function device (FFD) or a reduced-function device (RFD).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeviceType {
    /// A full-function device (FFD).
    FullFunctionDevice,
    /// A reduced-function device (RFD).
    ReducedFunctionDevice,
}
