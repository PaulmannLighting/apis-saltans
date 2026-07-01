use apis_saltans_core::units::Deciseconds;
use apis_saltans_zcl::Options;
use apis_saltans_zcl::general::level::Mode;
use smarthomelib::Percent;
use smarthomelib::protocol::Dimming;

use crate::Coordinator;

impl Dimming for Coordinator {
    type Rate = Deciseconds;
    type Step = Mode<u8>;

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
        .await
    }

    async fn step(
        &self,
        destination: Self::Destination,
        step: Self::Step,
        rate: Self::Rate,
    ) -> Result<(), Self::Error> {
        crate::Level::step(self, destination.into(), step, rate, Options::default()).await
    }

    async fn stop(&self, destination: Self::Destination) -> Result<(), Self::Error> {
        crate::Level::stop(self, destination.into(), Options::default()).await
    }
}
