use std::time::Duration;

use smarthomelib::Percent;
use smarthomelib::protocol::{Dimming, Types};
use zcl::Options;
use zigbee::types::Uint16;
use zigbee::{FromDeciSeconds, IntoDeciSeconds};

use crate::Coordinator;

impl Dimming for Coordinator {
    async fn dim(
        &self,
        destination: <Self as Types>::Destination,
        percent: Percent,
        rate: Duration,
    ) -> Result<Duration, Self::Error> {
        let deci_seconds: u16 = rate.into_deci_seconds().try_into().unwrap_or(u16::MAX);
        let deci_seconds: Uint16 = deci_seconds.try_into().unwrap_or(Uint16::MAX);

        crate::Level::move_to_level(
            self,
            destination.into(),
            percent.into(),
            deci_seconds,
            Options::default(),
        )
        .await?;

        Ok(Duration::from_deci_seconds(deci_seconds.as_u16()))
    }
}
