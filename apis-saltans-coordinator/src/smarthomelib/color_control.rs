use bunt::Xy;
use smarthomelib::Protocol;
use smarthomelib::protocol::ColorControl;
use apis_saltans_zcl::Options;
use apis_saltans_core::units::Deciseconds;

use crate::Coordinator;

impl ColorControl for Coordinator {
    type Color = Xy;
    type TransitionTime = Deciseconds;

    async fn move_to_color(
        &self,
        destination: <Self as Protocol>::Destination,
        color: Xy,
        transition_time: Self::TransitionTime,
    ) -> Result<(), Self::Error> {
        crate::ColorControl::move_to_xy(
            self,
            destination.into(),
            color.x(),
            color.y(),
            transition_time,
            Options::default(),
        )
        .await?;

        Ok(())
    }
}
