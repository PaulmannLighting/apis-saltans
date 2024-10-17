use serde::{Deserialize, Serialize};

/// A device type.
///
/// A Zigbee device is either a full-function device (FFD) or a reduced-function device (RFD).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum DeviceType {
    /// A full-function device (FFD).
    FullFunctionDevice,
    /// A reduced-function device (RFD).
    ReducedFunctionDevice,
}
