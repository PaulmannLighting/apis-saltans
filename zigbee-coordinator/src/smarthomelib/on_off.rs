use smarthomelib::protocol::OnOff;

use crate::Coordinator;

impl OnOff for Coordinator {
    async fn on(&self, destination: Self::Destination) -> Result<(), Self::Error> {
        crate::OnOff::on(self, destination.into()).await
    }

    async fn off(&self, destination: Self::Destination) -> Result<(), Self::Error> {
        crate::OnOff::off(self, destination.into()).await
    }

    async fn toggle(&self, destination: Self::Destination) -> Result<(), Self::Error> {
        crate::OnOff::toggle(self, destination.into()).await
    }
}
