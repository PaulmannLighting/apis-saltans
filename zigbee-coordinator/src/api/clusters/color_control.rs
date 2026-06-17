use macaddr::MacAddr8;
use zcl::Options;
use zcl::lighting::color_control::MoveToColor;
use zigbee::Endpoint;

use crate::transceiver::zcl::Handle;
use crate::{Coordinator, Error, NetworkManager};

/// Trait for Color Control cluster operations.
pub trait ColorControl {
    /// Move to the specified color (x, y) over the given transition time.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn move_to_xy(
        &self,
        ieee_address: MacAddr8,
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
        ieee_address: MacAddr8,
        endpoint: Endpoint,
        color_x: u16,
        color_y: u16,
        transition_time: u16,
        options: Options,
    ) -> Result<(), Error> {
        self.zcl
            .unicast_static_cluster(
                self.network_manager
                    .get_short_id_from_ieee_address(ieee_address)
                    .await?
                    .ok_or(Error::UnknownDevice(ieee_address))?,
                endpoint,
                MoveToColor::new(color_x, color_y, transition_time, options),
            )
            .await
    }
}
