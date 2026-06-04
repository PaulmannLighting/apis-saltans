use zcl::lighting::color_control::{Command, MoveToColor};
use zcl::{Cluster, Options};
use zigbee::Endpoint;
use zigbee_hw::{Error, Metadata};

use crate::Coordinator;
use crate::transmitter::{Handle, Payload};

/// Trait for Color Control cluster operations.
pub trait ColorControl {
    /// Move to the specified color (x, y) over the given transition time.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn move_to_xy(
        &self,
        short_id: u16,
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
        short_id: u16,
        endpoint: Endpoint,
        color_x: u16,
        color_y: u16,
        transition_time: u16,
        options: Options,
    ) -> Result<(), Error> {
        self.transmitter
            .unicast(
                short_id,
                endpoint,
                Metadata::for_cluster::<MoveToColor>(None, None),
                Payload::Zcl {
                    manufacturer_code: None,
                    payload: Cluster::ColorControl(Command::MoveToColor(MoveToColor::new(
                        color_x,
                        color_y,
                        transition_time,
                        options,
                    ))),
                },
            )
            .await
    }
}
