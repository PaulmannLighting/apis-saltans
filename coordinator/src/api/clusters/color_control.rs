use zb_core::Destination;
use zb_core::units::{Deciseconds, Mireds};
use zb_hw::HwResponse;
use zb_zcl::Options;
use zb_zcl::color_control::{MoveToColor, MoveToColorTemperature};

use crate::Error;
use crate::api::Zcl;

/// Trait for Color Control cluster operations.
///
/// Each method queues its command and returns an [`HwResponse`]. Await the returned response to
/// observe the hardware transmission result.
pub trait ColorControl {
    /// Move to the specified color (x, y) over the given transition time.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued. The returned
    /// [`HwResponse`] reports hardware transmission errors when awaited.
    fn move_to_xy(
        &self,
        destination: Destination,
        color_x: u16,
        color_y: u16,
        transition_time: Deciseconds,
        options: Options,
    ) -> impl Future<Output = Result<HwResponse, Error>> + Send;

    /// Move to the specified color temperature over the given transition time.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued. The returned
    /// [`HwResponse`] reports hardware transmission errors when awaited.
    fn move_to_color_temperature(
        &self,
        destination: Destination,
        color_temperature: Mireds,
        transition_time: Deciseconds,
        options: Options,
    ) -> impl Future<Output = Result<HwResponse, Error>> + Send;
}

impl<T> ColorControl for T
where
    T: Zcl + Sync,
{
    async fn move_to_xy(
        &self,
        destination: Destination,
        color_x: u16,
        color_y: u16,
        transition_time: Deciseconds,
        options: Options,
    ) -> Result<HwResponse, Error> {
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
    ) -> Result<HwResponse, Error> {
        self.transmit(
            destination,
            MoveToColorTemperature::new(color_temperature, transition_time, options),
        )
        .await
    }
}
