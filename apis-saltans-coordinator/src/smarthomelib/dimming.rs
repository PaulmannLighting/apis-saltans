use smarthomelib::Percent;
use smarthomelib::protocol::Dimming;
use apis_saltans_zcl::Options;
use apis_saltans_core::units::Deciseconds;

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
