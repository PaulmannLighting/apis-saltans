use zcl::Options;
use zcl::lighting::color_control::MoveToColor;
use zigbee::{Address, Endpoint};
use zigbee_hw::Error;

use crate::Coordinator;
use crate::transmitter::Handle;

/// Trait for Color Control cluster operations.
pub trait ColorControl {
    /// Move to the specified color (x, y) over the given transition time.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn move_to_xy(
        &self,
        address: Address,
        endpoint: Endpoint,
        color_x: u16,
        color_y: u16,
        transition_time: u16,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl ColorControl for Coordinator {
    async fn move_to_xy(
        &self,
        address: Address,
        endpoint: Endpoint,
        color_x: u16,
        color_y: u16,
        transition_time: u16,
        options: Options,
    ) -> Result<(), Error> {
        self.transmitter
            .unicast_zcl_native(
                address,
                endpoint,
                MoveToColor::new(color_x, color_y, transition_time, options),
            )
            .await
    }
}
