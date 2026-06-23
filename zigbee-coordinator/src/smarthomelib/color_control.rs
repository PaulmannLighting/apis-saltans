use std::time::Duration;

use smarthomelib::{ColorControl, Rgb, Xy};
use zcl::Options;
use zigbee::{FromDeciSeconds, IntoDeciSeconds};

use crate::Coordinator;

impl ColorControl for Coordinator {
    async fn move_to_color(
        &self,
        device: Self::DeviceId,
        endpoint: Self::EndpointId,
        color: Rgb,
        transition_time: Option<Duration>,
    ) -> Result<Option<Duration>, Self::Error> {
        let xy: Xy = color.into();
        let deci_seconds: u16 = transition_time
            .map(|transition_time| {
                transition_time
                    .into_deci_seconds()
                    .try_into()
                    .unwrap_or(u16::MAX)
            })
            .unwrap_or_default();
        crate::ColorControl::move_to_xy(
            self,
            device,
            endpoint,
            xy.x(),
            xy.y(),
            deci_seconds,
            Options::default(),
        )
        .await?;
        Ok(transition_time.map(|_transition_time| Duration::from_deci_seconds(deci_seconds)))
    }
}
