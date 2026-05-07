use bunt::Xy;
use zcl::lighting::color_control::MoveToColor;
use zcl::{HeaderFactory, Options};
use zigbee::Endpoint;

use crate::Error;
use crate::zcl::tx_rx::Transmitter;

/// Trait for Color Control cluster operations.
pub trait ColorControl {
    /// Move to the specified color (x, y) over the given transition time.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn move_to_xy(
        &self,
        pan_id: u16,
        endpoint: Endpoint,
        color: Xy,
        transition_time: u16,
        options: Options,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Move to the specified color (x, y) over the given transition time.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn move_to_color<T>(
        &self,
        pan_id: u16,
        endpoint: Endpoint,
        color: T,
        transition_time: u16,
        options: Options,
    ) -> impl Future<Output = Result<u8, Error>> + Send
    where
        T: Into<Xy>,
    {
        self.move_to_xy(pan_id, endpoint, color.into(), transition_time, options)
    }
}

impl<T> ColorControl for T
where
    T: Transmitter + Sync,
{
    async fn move_to_xy(
        &self,
        pan_id: u16,
        endpoint: Endpoint,
        color: Xy,
        transition_time: u16,
        options: Options,
    ) -> Result<u8, Error> {
        self.send(
            pan_id,
            endpoint,
            MoveToColor::new(color.x(), color.y(), transition_time, options)
                .frame(self.next_seq().await?),
        )
        .await
    }
}
