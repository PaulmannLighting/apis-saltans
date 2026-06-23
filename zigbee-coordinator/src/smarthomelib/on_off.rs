use smarthomelib::protocol::{OnOff, Types};

use crate::Coordinator;

impl OnOff for Coordinator {
    async fn on(&self, destination: <Self as Types>::Destination) -> Result<(), Self::Error> {
        crate::OnOff::on(self, destination.into()).await
    }

    async fn off(&self, destination: <Self as Types>::Destination) -> Result<(), Self::Error> {
        crate::OnOff::off(self, destination.into()).await
    }

    async fn toggle(&self, destination: <Self as Types>::Destination) -> Result<(), Self::Error> {
        crate::OnOff::toggle(self, destination.into()).await
    }
}
