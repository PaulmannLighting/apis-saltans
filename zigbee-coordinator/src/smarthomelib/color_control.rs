use std::time::Duration;

use smarthomelib::protocol::{ColorControl, Types};
use smarthomelib::{Deciseconds, Rgb, Xy};
use zcl::Options;
use zigbee::types::Uint16;

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
            .as_deci_secs()
            .try_into()
            .unwrap_or(u16::MAX);
        let deci_seconds: Uint16 = deci_seconds.try_into().unwrap_or(Uint16::MAX);

        crate::ColorControl::move_to_xy(
            self,
            destination.into(),
            xy.x(),
            xy.y(),
            deci_seconds,
            Options::default(),
        )
        .await?;

        Ok(Duration::from_deci_secs(deci_seconds.as_u16().into()))
    }
}
