use smarthomelib::TranslateEndpointId;

use crate::Coordinator;

impl TranslateEndpointId<u8> for Coordinator {
    async fn translate_endpoint_id(&self, endpoint: u8) -> Result<Self::EndpointId, Self::Error> {
        Ok(endpoint.into())
    }
}
