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
