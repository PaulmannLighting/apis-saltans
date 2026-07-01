use apis_saltans_zcl::general::on_off;
use apis_saltans_zcl::general::on_off::DyingLight;
use smarthomelib::command::{DelayedAllOff, Effect};
use smarthomelib::protocol::OnOff;

use crate::Coordinator;

impl OnOff for Coordinator {
    async fn on(&self, destination: Self::Destination) -> Result<(), Self::Error> {
        crate::OnOff::on(self, destination.into()).await
    }

    async fn off(&self, destination: Self::Destination) -> Result<(), Self::Error> {
        crate::OnOff::off(self, destination.into()).await
    }

    async fn off_with_effect(
        &self,
        destination: Self::Destination,
        effect: Effect,
    ) -> Result<(), Self::Error> {
        crate::OnOff::off_with_effect(self, destination.into(), effect.into()).await
    }

    async fn toggle(&self, destination: Self::Destination) -> Result<(), Self::Error> {
        crate::OnOff::toggle(self, destination.into()).await
    }
}
