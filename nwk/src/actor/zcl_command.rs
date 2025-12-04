use zcl::general::on_off::{Off, On};
use zcl::lighting::color_control::MoveToColor;

/// ZCL commands that can be sent to devices.
#[derive(Debug)]
pub enum ZclCommand {
    /// Turn the device on.
    On(On),
    /// Turn the device off.
    Off(Off),
    /// Move the device to a specific color.
    MoveToColor(MoveToColor),
}

impl From<On> for ZclCommand {
    fn from(cmd: On) -> Self {
        Self::On(cmd)
    }
}

impl From<Off> for ZclCommand {
    fn from(cmd: Off) -> Self {
        Self::Off(cmd)
    }
}

impl From<MoveToColor> for ZclCommand {
    fn from(cmd: MoveToColor) -> Self {
        Self::MoveToColor(cmd)
    }
}
