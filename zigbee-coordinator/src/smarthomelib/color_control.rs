use std::time::Duration;

use smarthomelib::protocol::{ColorControl, Types};
use smarthomelib::{Rgb, Xy};
use zcl::Options;
use zigbee::{FromDeciSeconds, IntoDeciSeconds};

use crate::Coordinator;

impl ColorControl for Coordinator {
    async fn move_to_color(
        &self,
        destination: <Self as Types>::Destination,
        color: Rgb,
        transition_time: Duration,
    ) -> Result<Duration, Self::Error> {
        let xy: Xy = color.into();
        let deci_seconds: u16 = transition_time
            .into_deci_seconds()
            .try_into()
            .unwrap_or(u16::MAX);

        crate::ColorControl::move_to_xy(
            self,
            destination.into(),
            xy.x(),
            xy.y(),
            deci_seconds,
            Options::default(),
        )
        .await?;

        Ok(Duration::from_deci_seconds(deci_seconds))
    }
}
