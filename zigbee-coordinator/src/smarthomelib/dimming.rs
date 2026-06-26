use smarthomelib::Percent;
use smarthomelib::protocol::Dimming;
use zcl::Options;
use zigbee::units::Deciseconds;

use crate::Coordinator;

impl Dimming for Coordinator {
    type Rate = Deciseconds;

    async fn dim(
        &self,
        destination: Self::Destination,
        percent: Percent,
        rate: Self::Rate,
    ) -> Result<(), Self::Error> {
        crate::Level::move_to_level(
            self,
            destination.into(),
            percent.into(),
            rate,
            Options::default(),
        )
        .await?;

        Ok(())
    }
}
