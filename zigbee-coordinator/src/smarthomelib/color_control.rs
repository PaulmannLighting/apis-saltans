use std::time::Duration;

use smarthomelib::protocol::{ColorControl, Types};
use smarthomelib::{Deciseconds, Rgb, Xy};
use zcl::Options;
use zigbee::types::Uint16;

use crate::Coordinator;
use crate::smarthomelib::constants::MAX_UINT16_DURATION;

impl ColorControl for Coordinator {
    const MAX_DELAY: Duration = MAX_UINT16_DURATION;

    async fn move_to_color(
        &self,
        destination: <Self as Types>::Destination,
        color: Rgb,
        transition_time: Duration,
    ) -> Result<(), Self::Error> {
        let xy: Xy = color.into();
        let transition_time_deci_secs: u16 = transition_time
            .as_deci_secs()
            .try_into()
            .map_err(|_| Self::Error::DurationOutOfBounds(transition_time))?;
        let transition_time_deci_secs: Uint16 = transition_time_deci_secs
            .try_into()
            .map_err(|_| Self::Error::DurationOutOfBounds(transition_time))?;

        crate::ColorControl::move_to_xy(
            self,
            destination.into(),
            xy.x(),
            xy.y(),
            transition_time_deci_secs,
            Options::default(),
        )
        .await?;

        Ok(())
    }
}
