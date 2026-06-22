use macaddr::MacAddr8;
use smarthomelib::{TranslateDeviceId, TranslateEndpointId};
use zigbee::Endpoint;

use crate::Coordinator;

impl TranslateEndpointId<(u16, Endpoint)> for Coordinator {
    async fn translate_endpoint_id(
        &self,
        (device, endpoint): (u16, Endpoint),
    ) -> Result<Self::EndpointId, Self::Error> {
        Ok((self.translate_device_id(device).await?, endpoint))
    }
}

impl TranslateEndpointId<(MacAddr8, u8)> for Coordinator {
    async fn translate_endpoint_id(
        &self,
        (device, endpoint): (MacAddr8, u8),
    ) -> Result<Self::EndpointId, Self::Error> {
        Ok((device, endpoint.into()))
    }
}

impl TranslateEndpointId<(u16, u8)> for Coordinator {
    async fn translate_endpoint_id(
        &self,
        (device, endpoint): (u16, u8),
    ) -> Result<Self::EndpointId, Self::Error> {
        Ok((self.translate_device_id(device).await?, endpoint.into()))
    }
}

impl TranslateEndpointId<MacAddr8> for Coordinator {
    async fn translate_endpoint_id(
        &self,
        device: MacAddr8,
    ) -> Result<Self::EndpointId, Self::Error> {
        Ok((device, Endpoint::default()))
    }
}

impl TranslateEndpointId<u16> for Coordinator {
    async fn translate_endpoint_id(&self, device: u16) -> Result<Self::EndpointId, Self::Error> {
        Ok((self.translate_device_id(device).await?, Endpoint::default()))
    }
}
