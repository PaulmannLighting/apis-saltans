use smarthomelib::protocol::OnOff;

use crate::Coordinator;

impl OnOff for Coordinator {
    async fn on(
        &self,
        device: Self::DeviceId,
        endpoint: Self::EndpointId,
    ) -> Result<(), Self::Error> {
        crate::OnOff::on(self, device, endpoint).await
    }

    async fn off(
        &self,
        device: Self::DeviceId,
        endpoint: Self::EndpointId,
    ) -> Result<(), Self::Error> {
        crate::OnOff::off(self, device, endpoint).await
    }

    async fn toggle(
        &self,
        device: Self::DeviceId,
        endpoint: Self::EndpointId,
    ) -> Result<(), Self::Error> {
        crate::OnOff::toggle(self, device, endpoint).await
    }
}
