use smarthomelib::protocol::OnOff;

use crate::Coordinator;

impl OnOff for Coordinator {
    async fn on(&self, endpoint: Self::EndpointId) -> Result<(), Self::Error> {
        crate::OnOff::on(self, endpoint.0, endpoint.1).await
    }

    async fn off(&self, endpoint: Self::EndpointId) -> Result<(), Self::Error> {
        crate::OnOff::off(self, endpoint.0, endpoint.1).await
    }

    async fn toggle(&self, endpoint: Self::EndpointId) -> Result<(), Self::Error> {
        crate::OnOff::toggle(self, endpoint.0, endpoint.1).await
    }
}
