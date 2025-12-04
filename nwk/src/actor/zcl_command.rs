use zcl::general::on_off::{Off, On};
use zcl::lighting::color_control::MoveToColor;
use zigbee::Endpoint;

use crate::{Error, Nlme};

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

impl ZclCommand {
    /// Execute the ZCL command using the provided NLME, PAN ID, and endpoint.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command execution fails.
    pub async fn execute<T>(
        self,
        nlme: &mut T,
        pan_id: u16,
        endpoint: Endpoint,
    ) -> Result<(), Error<T::Error>>
    where
        T: Nlme,
    {
        match self {
            Self::On(cmd) => nlme.unicast_command(pan_id, endpoint, cmd).await,
            Self::Off(cmd) => nlme.unicast_command(pan_id, endpoint, cmd).await,
            Self::MoveToColor(cmd) => nlme.unicast_command(pan_id, endpoint, cmd).await,
        }
    }
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
