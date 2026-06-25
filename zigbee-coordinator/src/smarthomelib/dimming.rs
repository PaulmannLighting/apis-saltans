use std::time::Duration;

use smarthomelib::protocol::{Dimming, Types};
use smarthomelib::{Deciseconds, Percent};
use zcl::Options;
use zigbee::types::Uint16;

use crate::Coordinator;
use crate::smarthomelib::constants::MAX_UINT16_DURATION;

impl Dimming for Coordinator {
    const MAX_DIM_RATE: Duration = MAX_UINT16_DURATION;

    async fn dim(
        &self,
        destination: <Self as Types>::Destination,
        percent: Percent,
        rate: Duration,
    ) -> Result<(), Self::Error> {
        let rate_deci_secs: u16 = rate
            .as_deci_secs()
            .try_into()
            .map_err(|_| Self::Error::DurationOutOfBounds(rate))?;
        let rate_deci_secs: Uint16 = rate_deci_secs
            .try_into()
            .map_err(|_| Self::Error::DurationOutOfBounds(rate))?;

        crate::Level::move_to_level(
            self,
            destination.into(),
            percent.into(),
            rate_deci_secs,
            Options::default(),
        )
        .await?;

        Ok(())
    }
}
