use zb_core::Destination;
use zb_core::units::{Deciseconds, Mireds};
use zb_zcl::Options;
use zb_zcl::color_control::{MoveToColor, MoveToColorTemperature};

use crate::transceiver::zcl::Handle;
use crate::{Coordinator, Error};

/// Trait for Color Control cluster operations.
pub trait ColorControl {
    /// Move to the specified color (x, y) over the given transition time.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn move_to_xy(
        &self,
        destination: Destination,
        color_x: u16,
        color_y: u16,
        transition_time: Deciseconds,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Move to the specified color temperature over the given transition time.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn move_to_color_temperature(
        &self,
        destination: Destination,
        color_temperature: Mireds,
        transition_time: Deciseconds,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl ColorControl for Coordinator {
    async fn move_to_xy(
        &self,
        destination: Destination,
        color_x: u16,
        color_y: u16,
        transition_time: Deciseconds,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(
            destination,
            MoveToColor::new(color_x, color_y, transition_time, options),
        )
        .await
    }

    async fn move_to_color_temperature(
        &self,
        destination: Destination,
        color_temperature: Mireds,
        transition_time: Deciseconds,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(
            destination,
            MoveToColorTemperature::new(color_temperature, transition_time, options),
        )
        .await
    }
}
